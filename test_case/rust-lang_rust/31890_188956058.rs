 rust
pub fn imperative_add_vec_vec_zip(a: &Vec<i32>, b: &Vec<i32>, c: &mut Vec<i32>) {
    for ((a, b), c) in a.iter().zip(b).zip(c) {
        *c = *a + *b;
    }
}
