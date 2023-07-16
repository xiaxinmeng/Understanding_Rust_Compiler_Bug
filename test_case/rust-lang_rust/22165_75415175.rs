 rust
fn test<I: Iterator<Item=i32>>(i: I) -> Vec<i32> {
    let v = i.collect();
    v.push(1); //~ error: the type of this value must be known in this context
    v
}
