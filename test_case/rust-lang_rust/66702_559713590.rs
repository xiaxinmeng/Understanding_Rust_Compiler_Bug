rust
fn main() {
    let foo = vec![1, 2, 3];
    'some_label: loop {
        foo.map(|x| { break 'some_label 0; });
    }
}
