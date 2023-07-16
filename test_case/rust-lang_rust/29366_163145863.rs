
fn foo(opt: Option<Bar>) -> Bar {
    let bar = opt.unwrap_or(return Bar::new());
    bar.baz()
}
