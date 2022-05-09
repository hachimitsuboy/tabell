# tabell
ネーミングコマンドラインツール
https://coveralls.io/repos/github/hachimitsuboy/tabell/badge.svg(Coverage Status)!:https://coveralls.io/github/hachimitsuboy/tabell



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



