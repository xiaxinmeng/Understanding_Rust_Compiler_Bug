rust
macro_rules! foo {
    ($e:expr) => {
        bar!($e);
        baz!($e);
    }
}
macro_rules! bar {
    ($e:expr) => { if !$e { panic!(); } }
}
macro_rules! baz {
    ($e:expr) => { if $e { panic!(); }  }
}

fn main() {
    foo!(true);
}
