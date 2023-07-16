 rust
#![feature(macro_rules)]

trait Seq<T> {
    fn with_capacity(capacity: uint) -> Self;
    // HACK: A method doesn't play well with the inference engine, use a static function instead
    fn add_elem(seq: &mut Self, elem: T);
}

// FIXME (rust-lang/rfcs#88) Use the `$#($arg)` syntax instead of the `count_args!` macro
macro_rules! count_args {
    () => { 0 };
    ($x:expr) => { 1 };
    ($head:expr, $($tail:expr),+) => { 1 + count_args!($($tail),+) };
}

macro_rules! seq {
    // List style: Vec, {Hash,Tree}Set, etc
    ($($x:expr),*) => ({
        let mut _temp = Seq::with_capacity(count_args!($($x),*));

        $(Seq::add_elem(&mut _temp, $x);)*

        _temp
    });
    // Map style: {Hash,Tree}Map, etc
    ($($k:expr => $v:expr),*) => ({
        let mut _temp = Seq::with_capacity(count_args!($(($k, $v)),*));

        $(Seq::add_elem(&mut _temp, ($k, $v));)*

        _temp
    });
    // Trailing comma <3
    ($($x:expr),+,) => { seq!($($x),+) };
    ($($k:expr => $v:expr),+,) => { seq!($($k => $v),+) };
}
