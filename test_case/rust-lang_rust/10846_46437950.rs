
#[cfg(show_erickt_bug)]
fn erickt_fn() {
    fn test2(_x: Option<|y: Option<|z: &int|>|>) {}
    test2(Some(|_y: Option<|z: &int|>| {}));
}
#[cfg(not(show_erickt_bug))]
fn erickt_fn() {
    type F1<'a> = |z: &'a int|:'a;
    type F2<'b> = |y: Option<F1<'b>>|:'b;
    fn test2<'c>(_x: Option<F2>) {}
    test2(Some(|_y| {}))
}
