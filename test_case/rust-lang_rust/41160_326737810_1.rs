rust
pub fn write_to_array(to: &mut [u64;30]) {
    let v = [55;30];
    *to = v;
}
