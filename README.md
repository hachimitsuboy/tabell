# tabell
ネーミングコマンドラインツール

<p align="center">
  <img src="https://user-images.githubusercontent.com/39763423/179402936-f2798e3e-2d62-479a-a7ea-d7103cc8a87e.png" width = "180px" />
</p>

<img src="https://img.shields.io/badge/-Rust-8b4513.svg?logo=rust&style=popout">

[![Coverage Status](https://coveralls.io/repos/github/hachimitsuboy/tabell/badge.svg?branch=main)](https://coveralls.io/github/hachimitsuboy/tabell?branch=main)
[![build](https://github.com/hachimitsuboy/tabell/actions/workflows/build.yml/badge.svg)](https://github.com/hachimitsuboy/tabell/actions/workflows/build.yml)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/hachimitsuboy/tabell)](https://rust-reportcard.xuri.me/report/github.com/hachimitsuboy/tabell)
[![DOI](https://zenodo.org/badge/490134626.svg)](https://zenodo.org/badge/latestdoi/490134626)
![](https://img.shields.io/badge/novice-engineer-success)










## Description
プログラム作成中など変数名を決める際、使用用途に応じて名付けるが、日本語では思いついてもコードの中では変数名は英語で書かなくてはならないが、英語名がパッと思い浮かばない。
そんな経験はないだろうか？　このコマンドラインツールを使用すれば、変数名を決める手間を省くことができる。
コマンドラインに、変数名にしたい日本語を入力すると、英語に変換し出力される。また、「大きな時計」のような複合語の場合、「largeClock」という様に、キャメルケースで出力される。

## Usage

```

tabell [OPTIONS] <MESSAGE>

OPTIONS:
    -h, --help             Print help information
    -k, --kabab <KABAB>    generate variable names with kebab case
    -s, --snake <SNAKE>    generate variable names with snake case
    -V, --version          Print version information
    
```

## Sample Output
 
```

$tabell 大きな時計
  
largeClock

```



