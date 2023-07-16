rust
fn concat_arrs<T, const N: usize, const M: usize>(a: &mut ([T; N], [T; M])) -> &mut [T; N + M]; // Without coping, same for 3-tuples, 4-tuples, etc
fn concat_arr_of_arrs<T, const N: usize, const M: usize>(a: &mut [[T; N]; M]) -> &mut [T; N * M];
