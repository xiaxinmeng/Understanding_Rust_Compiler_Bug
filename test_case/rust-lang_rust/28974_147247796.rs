 rust


macro_rules! foo {
    ( $e:expr) => { ($e) }
}

macro_rules! bar {
    ( < $x:ident > ) => { $x };
    ( $x:tt ) => { foo!($x) };
}

macro_rules! baz {
    ( $ ( $( $x:tt )|*)* ) => {
        {
            make($n, &[$(&[$(bar!($x)
        ),*]),*])
        }
    };
}


fn main() {
    let bar = baz!["0","1","2","3","4","5","6","7","8","9"];
}
