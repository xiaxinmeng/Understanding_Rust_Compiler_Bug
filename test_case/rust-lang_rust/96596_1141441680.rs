
// CHECK-LABEL: @vec_one_array_32
#[no_mangle]
pub fn vec_one_array_32(n: usize) -> Vec<[i64; 32]> {
    vec![[1_i64; 32]; n]
}
