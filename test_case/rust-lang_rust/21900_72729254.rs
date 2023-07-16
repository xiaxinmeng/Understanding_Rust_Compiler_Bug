 rust
// this passes the assertion
fn main() {
    let mut v = Vec::new();
    for i in 0..10 {
        let index = v.len();
        v.insert(index, i);
    }

    let mut u = Vec::new();
    for i in 0..10 {
        u.push(i)
    }
    assert_eq!(u, v);
}
