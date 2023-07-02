use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// リネームされるファイルやディレクトリへのパスを表す。
    /// このパスは絶対パスでも相対パスでもよい。
    path: String,
}

use std::ffi;
use std::path;

fn main() {
    let args = Args::parse();
    let path = path::Path::new(&args.path).canonicalize().unwrap();
    println!("{}", path.display());

    let newpath = build_newpath(&path);
    println!("{}", newpath.display());

    println!("{} {}", path.display().to_string().len(), newpath.display().to_string().len());
}

/// UTF-8表現におけるファイルやディレクトリの名前の長さの上限。
const NAME_MAX: usize = 255;
const CHECKSUM_LENGTH: usize = 4;

/// パスを短縮済みのパスに変換する。
fn build_newpath(path: &path::PathBuf) -> path::PathBuf {
    if path.is_file() {
        let filename = path.file_name().expect("path must have filename");
        let filename = ffi::OsString::from(&filename);
        let filename = filename.into_string().expect("invalid Unicode data: filename");
        let cksum = calculate_checksum(&filename);

        let ext = path.extension().expect("path must have extension");
        let ext = ffi::OsString::from(&ext);
        let ext = ext.into_string().expect("invalid Unicodedata: ext");

        let limit = NAME_MAX - CHECKSUM_LENGTH - 1 - ext.len();
        let stem = path.file_stem().expect("path must have stem");
        let stem = ffi::OsString::from(&stem);
        let stem = stem.into_string().expect("invalid Unicode data: stem");
        let shortstem = shorten_stem(&stem, limit);

        assert!(shortstem.len() + cksum.len() + 1 + ext.len() <= NAME_MAX);

        let mut newfilename = ffi::OsString::new();
        newfilename.push(shortstem);
        newfilename.push(cksum);
        newfilename.push(".");
        newfilename.push(ext);

        let mut newpath = path.clone();
        newpath.set_file_name(newfilename);
        newpath
    } else if path.is_dir() {
        todo!()
    } else {
        panic!("neither file nor directory")
    }
}

use unicode_segmentation::UnicodeSegmentation;

/// ステムを受けとり、limitバイトに収まるように末尾を切り落とす。
/// 切り落とす単位は書記素クラスタとする。
fn shorten_stem(stem: &str, limit: usize) -> &str {
    // limitに収まる境界を探す
    let mut boundary = 0;
    for gc in stem.graphemes(true) {
        let nb = gc.len();
        if boundary + nb >= limit {
            break;
        }
        boundary += nb;
    }

    &stem[..boundary]    
}

use bytemuck::cast_slice;
use fletcher::calc_fletcher32;
/// 文字列のFletcher-32チェックサムを計算して16進表現で返す。
fn calculate_checksum(s: &str) -> String {
    let cksum = calc_fletcher32(cast_slice(s.as_bytes()));
    let ckbytes = cksum.to_be_bytes();
    format!("{:x}{:x}{:x}{:x}", ckbytes[0], ckbytes[1], ckbytes[2], ckbytes[3])
}