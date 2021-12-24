# minimal_input

[proconio](https://docs.rs/proconio) が使えない競技プログラミング等で使う、Rust で標準入力をパースするライブラリである。

[src/lib.rs](src/lib.rs) 全体をそのまま貼り付け、必要であれば `use minimal_input::marker::Usize1;` などを書けば、proconio と同様に使うことができる。短いコードでほぼ同等の機能を自走している

相違点を列挙しておくが、他にも細かい違いがあるかもしれない。

- モジュール名が異なる
  - ソースコード中の `minimal_input` を `proconio` に置換すれば同じになる
- proconio では lazy_static を使っているが、minimal_input では thread_local を使っており、標準ライブラリ以外への依存がない
- proconio では行単位で入力を受け取る（LineSource）か一気に入力を受け取る（OnceSource）か選ぶことができるが、minimal_input では行単位で固定である
  - 一気に入力したい場合は、[23行目](src/lib.rs#L23) の `read_line` を `read_to_string` に書き換える
- Source の実装が異なる
  - Source は trait ではなく struct であり、Stdin 以外からの入力はできない
  - Source は is_empty を実装せず、すなわち is_stdin_empty も実装しない
- fastout は存在しない

## ライセンス

このライブラリは、MIT License のもとで公開されている。
ただし、競技プログラミング等のためにこのライブラリをジャッジサーバ等に送信するとき、著作権表示および許諾表示を省略することができる。

ここで、「競技プログラミング等」には、[アルゴ式](https://algo-method.com/)などのプログラミング学習サービスを含む。
