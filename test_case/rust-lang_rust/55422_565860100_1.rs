
fn foo<'a>(src: Box<&'static ()>) -> &'a () {
    *src
}
