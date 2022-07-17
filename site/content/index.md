# Tabell

# 

**ネーミングコマンドラインツール**  

## **Description (説明)**

プログラム作成中など変数名を決める際、使用用途に応じて名付けるが、日本語でその変数の役割が考えられても、その役割が伝わるような英語名がパッと思い浮かばない。 そんな経験はないだろうか？　このコマンドラインツールを使用すれば、変数名を決める手間を省くことができる。 コマンドラインに、変数の役割にちなんだ日本語の単語を入力すると、英語に変換し出力される。また、「大きな時計」のような複合語の場合、「largeClock」という様に、キャメルケースで出力される。

## **Usage (使い方)**

```jsx
tabell [OPTIONS] <MESSAGE>

OPTIONS:
    -h, --help             Print help information
    -k, --kabab <KABAB>    generate variable names with kebab case
    -s, --snake <SNAKE>    generate variable names with snake case
    -V, --version          Print version information
```

## **Sample Output (出力例)**

```jsx
$tabell 大きな時計
>largeClock
```