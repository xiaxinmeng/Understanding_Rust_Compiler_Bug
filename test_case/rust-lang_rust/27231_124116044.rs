 rust
struct S(u8);

macro_rules! mkexpr {
    ( $e:expr ) => ( $e );
}

macro_rules! mac {
    //( $val:tt ) => ( S($val) );   // Doesn't work
    ( $val:tt ) => ( S(mkexpr!($val)) );
}

fn main() {
    let s: S = mac!(4);
    println!("{}", s.0);
}
