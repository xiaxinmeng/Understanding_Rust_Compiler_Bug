rust
fn main() {
    let mut v = vec![""];
    v.push({
        let x = v.pop().unwrap();
        x
    });
}
