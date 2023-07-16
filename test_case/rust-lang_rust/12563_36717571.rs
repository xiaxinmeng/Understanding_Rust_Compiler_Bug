 rust
fn main() {
    let mut foo = ~[];
    'foo: for i in [1, 2, 3].iter() {
        foo.push(i);
    }
}
