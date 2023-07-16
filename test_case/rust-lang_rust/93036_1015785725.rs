rust
pub fn f(arr: &mut [u32]) {
    for i in 0..arr.len() {
        let subslice = &arr[0..i];
        for j in 0..subslice.len() {
            assert!(j < subslice.len());
        }
    }
}
