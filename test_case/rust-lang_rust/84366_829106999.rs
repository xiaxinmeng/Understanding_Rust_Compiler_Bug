rust
fn foo<'a, F: Future<Output = &'a str> + 'static>(f: F) {}

fn bar<'a>() {
    let fut = async { "" };
    foo::<'a>(fut);
}
