# shortname
与えられたファイルやディレクトリの名前をUTF-8換算で255バイトに収まるようにリネームします。

## 使いかた
```bash
shortname your_file_or_directory
```

## なぜこのコマンドが必要か
多くのUNIX系OSでは、ファイルやディレクトリの名前を任意のバイト列(大抵はUTF-8)として扱い、その長さの上限を**255バイト**までとしている。
しかしWindowsでは、ファイルやディレクトリの名前をUTF-16として扱い、その長さの上限は**260文字**である。
そのため、Windowsでは長い名前を許容する文化があり、そのままではUNIX系システムとファイルをやりとりすることができない。
ここでは、名前を切り詰めることでこの問題の回避を試みる。

## 名前形式の仕様

### ファイルの場合
次の要素を区切り文字なしで連結し、UTF-8でエンコードしたものとする。
1. 短縮名(shortstem)
2. チェックサム(checksum)
3. ドット/ピリオド
4. オリジナルの末尾の拡張子(extension)

これらの要素は255バイトの領域に**後ろ**から、つまり4番目の拡張子から詰めていく。
短縮名の長さの上限は次の計算式で求める。

`255 - チェックサムの長さ4 - ドット/ピリオドの長さ1 - オリジナルの末尾の拡張子の長さ`

### ディレクトリの場合
次の要素を区切り文字なしで連結し、UTF-8でエンコードしたものとする。
1. 短縮名(shortstem)
2. チェックサム(checksum)

これらの要素は255バイトの領域に**後ろ**から、つまり2番目のチェックサムから詰めていく。
短縮名の長さの上限は次の計算式で求める。

`255 - チェックサムの長さ4`

### 短縮名
オリジナルから末尾の拡張子を取り除いたものをステム(stem)と呼ぶ。  
ステムのUTF-8表現の長さが、短縮名の長さの上限を超えなくなるまで、書記素クラスタ単位で末尾から切り落とす。

### チェックサム
チェックサムは、オリジナルのファイル名をUTF-8として表したときの32ビットチェックサムである。
現在はFletcher-32を用いて計算する。

## 実装上の注意
WindowsではUTF-16でファイルやディレクトリの名前を扱うため、入出力の際にはUTF-16とUTF-8の相互変換が必要である。
