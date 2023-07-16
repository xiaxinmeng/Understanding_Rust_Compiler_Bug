rust
pub fn foo() -> Vec<usize> {
    let mut arr: Vec<usize> = vec![];
    arr.resize_with(100, || Default::default());
    assert_eq!(arr.len(), 100);
    for i in 0 .. 100 {
        arr[i] = i;
    }
    arr
}
