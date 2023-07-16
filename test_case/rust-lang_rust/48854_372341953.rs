rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn e() {
    let a = [1, 2, 100];
    let b = a[3]; // oops
    let c = t(&a, b);
}

fn t(a: &[usize], idx: usize) {
    a[idx];
}
