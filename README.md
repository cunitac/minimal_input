# minimal_input

[proconio](https://docs.rs/proconio) が使えない競技プログラミング等で使う、Rust で標準入力をパースするライブラリである。

[src/lib.rs](src/lib.rs) 全体をそのまま貼り付け、必要であれば `use minimal_input::marker::Usize1;` などを書けば、proconio と同様に使うことができる。

相違点を列挙しておくが、他にも細かい違いがあるかもしれない。

- モジュール名が異なる
  - ソースコード中の `minimal_input` を `proconio` に置換すれば同じになる
- proconio では lazy_static を使っているが、minimal_input では thread_local を使っており、標準ライブラリ以外への依存がない
  - つまり、スレッド安全でない
- proconio では `from source` として入力を指定することができるが、minimal_input ではできない（できるようにすることも検討している） 
- proconio では行単位で入力を受け取る（LineSource）か一気に入力を受け取る（OnceSource）か選ぶことができるが、minimal_input では行単位で固定である
  - 一気に入力したい場合は、[23行目](src/lib.rs#L23) の `read_line` を `read_to_string` に書き換える
- minimal_input の実装はかなり短い（約100行）
- minimal_input には `read!` マクロが存在する
  - `read!(u32, [i32])` は `{ input!(x: (u32, [i32])); x }` と等価
  - `read!(@source, u32)` は proconio の `{ input!(from source, x: u32); x }` に相当
- Source の実装が異なり、そのため Readable の実装方法が異なる
  - Source が実装するメソッドは next_token のみであり、これは proconio::source:Source の next_token_unwrap に相当する
  - 前述のように `from source` が存在しないため、代わりに `read!` マクロを用いる
- fastout は存在しない

## ライセンス

このライブラリは、MIT License のもとで公開されている。
ただし、競技プログラミング等のためにこのライブラリをジャッジサーバ等に送信するとき、著作権表示および許諾表示を省略することができる。

ここで、「競技プログラミング等」には、[アルゴ式](https://algo-method.com/)などのプログラミング学習サービスを含む。
