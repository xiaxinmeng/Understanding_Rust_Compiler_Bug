rs
fn main() {
    let mut m = std::collections::HashMap::new();
    //  ^^^^^ error: type annotations needed for `HashMap<i32, i32, S>`
    m.insert(0, 1);
    dbg!(m);
}
