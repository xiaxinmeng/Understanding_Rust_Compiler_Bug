rust
macro_rules! foo {
    ($(),* $($bar:ident)?) => {};
//       ^ (6)
}

foo! {
    bar
}
