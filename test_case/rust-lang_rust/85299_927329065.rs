rust
pub fn mat_total(mat: &[Vec<i32>]) -> i32 {
    let mut tot = 0;
    for i in 0 .. mat.len() {
        let v = mat[i].as_slice();
        for j in 0 .. v.len() {
            tot += v[j];
        }
    }
    tot
}
