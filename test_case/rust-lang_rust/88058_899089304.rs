rs
fn iter<'a>(x: Iter<'static, &'static (), &'static ()>) -> Iter<'a, &'a (), &'a ()> {
    x
}
