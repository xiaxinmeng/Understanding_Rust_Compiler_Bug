rust
fn foo() -> Vec<u32> {
    let v = vec![];
    v.extend(vec![1, 2, 3]);
    v.extend(other_vec_func());
    v
}
