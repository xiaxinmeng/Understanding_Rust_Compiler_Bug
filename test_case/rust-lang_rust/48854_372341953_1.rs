rust
#[test]
fn e() {
    let a = [1, 2, 100];
    let b = a[3]; // oops
    assert_that(|| {
        let c = t(&a, b);
    }).panics()
        .with("index out of bounds")
}
