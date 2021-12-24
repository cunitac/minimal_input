pub mod minimal_input {
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::io::{stdin, BufRead, BufReader, Stdin};
    use std::str::{FromStr, SplitWhitespace};
    thread_local!(
        pub static STDIN_SOURCE: RefCell<Source> = RefCell::new(Source {
            stdin: BufReader::new(stdin()),
            tokens: "".split_whitespace(),
        });
    );
    pub struct Source {
        stdin: BufReader<Stdin>,
        tokens: SplitWhitespace<'static>,
    }
    impl Source {
        pub fn next_token(&mut self) -> Option<&str> {
            self.tokens.next().or_else(|| {
                let mut input = String::new();
                self.stdin.read_line(&mut input).unwrap();
                self.tokens = Box::leak(input.into_boxed_str()).split_whitespace();
                self.tokens.next()
            })
        }
        pub fn next_token_unwrap(&mut self) -> &str {
            self.next_token().unwrap()
        }
    }
    #[macro_export]
    macro_rules! read_value {
        (from $s:expr, [$t:tt; $n:expr]) => {
            (0..$n).map(|_| $crate::read_value!(from $s, $t)).collect::<::std::vec::Vec<_>>()
        };
        (from $s:expr, [$t:tt]) => {{
            let n = $crate::read_value!(from $s, usize);
            $crate::read_value!(from $s, [$t; n])
        }};
        (from $s:expr, ($($t:tt),* $(,)?)) => {
            $crate::read_value!(from $s, $($t),*)
        };
        (from $s:expr, $t:ty) => {
            <$t as $crate::minimal_input::Readable>::read(&mut $s)
        };
        (from $s:expr, $($t:tt),* $(,)?) => {
            ($($crate::read_value!(from $s, $t)),*)
        };
        ($($r:tt)*) => {
            $crate::minimal_input::STDIN_SOURCE.with(|s| {
                let mut s = s.borrow_mut();
                $crate::read_value!(from s, $($r)*)
            })
        }
    }
    #[macro_export]
    macro_rules! input {
        () => {};
        ($x:tt: $t:tt, $($r:tt)*) => {
            let $x = $crate::read_value!($t);
            $crate::input!($($r)*);
        };
        (mut $x:tt: $t:tt, $($r:tt)*) => {
            let mut $x = $crate::read_value!($t);
            $crate::input!($($r)*);
        };
        (from $s:expr, $x:tt: $t:tt, $($r:tt)*) => {
            let $x = $crate::read_value!(from $s, $t);
            $crate::input!(from $s, $($r)*);
        };
        (from $s:expr, mut $x:tt: $t:tt, $($r:tt)*) => {
            let mut $x = $crate::read_value!(from $s, $t);
            $crate::input!(from $s, $($r)*);
        };
        ($($r:tt)*) => {
            $crate::input!($($r)*,);
        };
    }
    pub trait Readable {
        type Output;
        fn read(source: &mut Source) -> Self::Output;
    }
    impl<T: FromStr<Err = E>, E: Debug> Readable for T {
        type Output = T;
        fn read(source: &mut Source) -> T {
            source.next_token_unwrap().parse().unwrap()
        }
    }
    pub mod marker {
        macro_rules! impl_readable {
            ($e:ty, $t:ty, $u:ty, $f:expr) => {
                impl $crate::minimal_input::Readable for $e {
                    type Output = $t;
                    fn read(mut source: &mut $crate::minimal_input::Source) -> $t {
                        $f($crate::read_value!(from source, $u))
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
}
