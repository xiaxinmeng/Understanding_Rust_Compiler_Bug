 Rust
fn main() {
    let v = vec![];
    let _s = if true {
        &[]
    } else {
        &v[..]
    };
}
