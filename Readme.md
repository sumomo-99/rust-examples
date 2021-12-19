# Rust Examples
Rust勉強用
## bin2dec
2進数を10進数に変換します。
### 実行例
```
cargo run -p bin2dec -- 1101
2進数: 1101 -> 10進数: 13
```
## rcat
引数に指定したテキストファイルの内容を表示します。
```
USAGE:
    rcat [FLAGS] [path]...

FLAGS:
    -h, --help       Prints help information
    -n, --number     Prints line number
    -V, --version    Prints version information
```
### 実行例
```
cargo run -p rcat -- -n Cargo.toml Cargo.lock
```
## rgrep
引数に指定したパターンでファイルの内容を検索します。
```
USAGE:
    rgrep [FLAGS] <PATTERN> [FILE]...

FLAGS:
    -h, --help           Prints help information
    -n, --line-number    Prints line number
    -V, --version        Prints version information
```
### 実行例
```
cargo run -p rcat -- -n Cargo.toml Cargo.lock
```