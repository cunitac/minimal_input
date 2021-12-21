# minimal_input

[proconio](https://docs.rs/proconio) が使えない競技プログラミング等で使う、Rust で標準入力をパースするライブラリである。

基本的な使い方は proconio と全く同じである。相違点を列挙しておくが、他にも細かい違いがあるかもしれない。

- proconio では lazy_static を使っているが、minimal_input では thread_local を使っており、標準ライブラリへの依存がない
- proconio では Stdin 以外からも入力を受け取ることができるが、minimal_input では Stdin のみで、たとえば StdinLock も不可
- proconio では行単位で入力を読み込むか一気に入力を受け取るか選ぶことができるが、minimal_input ではできない
- minimal_input の実装はかなり短い（約100行）

## ライセンス

このライブラリは、MIT License のもとで公開されている。
ただし、競技プログラミング等のためにこのライブラリをジャッジサーバ等に送信するとき、著作権表示および許諾表示を省略することができる。

ここで、「競技プログラミング等」には、[アルゴ式](https://algo-method.com/)などのプログラミング学習サービスを含む。