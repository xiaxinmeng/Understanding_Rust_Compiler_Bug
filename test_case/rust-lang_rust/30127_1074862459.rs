rust
use std::ops::Index;

fn main() {
    let _sugar = &"a".to_owned()[..];
    let _desugar1 = "a".to_owned().index(..);
    let _desugar2 = &*"a".to_owned().index(..);
    println!("{:?}", _desugar2);
}
