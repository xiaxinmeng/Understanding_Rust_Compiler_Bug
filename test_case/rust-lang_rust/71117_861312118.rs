rust
fn main() {
    let a = [0; 128];
    loop {
        f(a);
    }
}

fn f(mut a: [u8; 128]) {
    assert_eq!(a[0], 0);
    a[0] += 1;
}
