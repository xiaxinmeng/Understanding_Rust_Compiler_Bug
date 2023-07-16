rust
macro_rules! foo {
    ($ident:ident) => { let $ident = 42; }
}

fn main() {
    foo!(bar);
    bar.pow(2);
}
