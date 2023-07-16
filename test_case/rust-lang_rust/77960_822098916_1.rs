rust
#[inline]
fn bar<S: Into<String>>(s: S) {
    some_work();
    more_work();
    let s: String = s.into();
    bar_impl(s);
}

fn bar_impl(s: String) {
    even_more_work(&s);
    yet_even_more_work(s);
}
