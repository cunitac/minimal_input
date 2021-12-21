//! proconio のかわりにコピペで使えるライブラリ

use std::cell::RefCell;
use std::fmt::Debug;
use std::io::{stdin, BufRead, BufReader, Stdin};
use std::str::{FromStr, SplitWhitespace};

thread_local!(
    pub static STDIN: RefCell<Source> = RefCell::new(Source {
        stdin: BufReader::new(stdin()),
        tokens: "".split_whitespace(),
    });
);

pub struct Source {
    stdin: BufReader<Stdin>,
    tokens: SplitWhitespace<'static>,
}

impl Source {
    pub fn next_token(&mut self) -> &str {
        self.tokens.next().unwrap_or_else(|| {
            let mut input = String::new();
            self.stdin.read_line(&mut input).unwrap();
            self.tokens = Box::leak(input.into_boxed_str()).split_whitespace();
            self.tokens.next().unwrap()
        })
    }
}

#[macro_export]
macro_rules! read {
    (@$s:expr, [$t:tt; $n:expr]) => {
        (0..$n).map(|_| $crate::read!(@$s, $t)).collect::<::std::vec::Vec<_>>()
    };
    (@$s:expr, [$t:tt]) => {{
        let n = $crate::read!(@$s, usize);
        $crate::read!(@$s, [$t; n])
    }};
    (@$s:expr, ($($t:tt),* $(,)?)) => {
        $crate::read!(@$s, $($t),*)
    };
    (@$s:expr, $t:ty) => {
        <$t as $crate::Readable>::read(&mut $s)
    };
    (@$s:expr, $($t:tt),* $(,)?) => {
        ($($crate::read!(@$s, $t)),*)
    };
    ($(r:tt)*) => {
        $crate::STDIN.with(|s| {
            let mut s = s.borrow_mut();
            $crate::read!(@s, $($r)*)
        })
    }
}

#[macro_export]
macro_rules! input {
    () => {};
    ($x:tt: $t:tt, $($r:tt)*) => {
        let $x = read!($t);
        $crate::input!($($r)*);
    };
    (mut $x:tt: $t:tt, $($r:tt)*) => {
        let mut $x = $crate::read!($t);
        $crate::input!($($r)*);
    };
    ($($r:tt)*) => {
        input!($($r)*,);
    };
}

pub trait Readable {
    type Output;
    fn read(source: &mut Source) -> Self::Output;
}

impl<T: FromStr<Err = E>, E: Debug> Readable for T {
    type Output = T;
    fn read(source: &mut Source) -> T {
        source.next_token().parse().unwrap()
    }
}

pub mod marker {
    macro_rules! impl_readable {
        ($e:ty, $t:ty, $u:ty, $f:expr) => {
            impl $crate::Readable for $e {
                type Output = $t;
                fn read(mut source: &mut $crate::Source) -> $t {
                    $f($crate::read!(@source, $u))
                }
            }
        }
    }
    pub enum Usize1 {}
    pub enum Isize1 {}
    pub enum Chars {}
    pub enum Bytes {}
    impl_readable!(Usize1, usize, usize, |x| x - 1);
    impl_readable!(Isize1, isize, isize, |x| x - 1);
    impl_readable!(Chars, Vec<char>, String, |x: String| x.chars().collect());
    impl_readable!(Bytes, Vec<u8>, String, |x: String| x.bytes().collect());
}
