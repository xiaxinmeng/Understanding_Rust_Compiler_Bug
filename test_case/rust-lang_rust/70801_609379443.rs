rust
fn simple(x: for<'a> fn(&'a ())) -> fn(&'static ()) {
    x
}

fn nested(x: (for<'a> fn(&'a ()), String)) -> (fn(&'static ()), String) {
    x
}
