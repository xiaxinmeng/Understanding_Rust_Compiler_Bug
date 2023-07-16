 rust
let a: i32 = ...;

macro_rules! foo {
    ($e: expr) => { quote_expr!(cx, $$a + $e) }
}

foo!(1);
foo!(2);
