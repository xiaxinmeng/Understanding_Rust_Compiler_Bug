rust
pub fn compute_rank(mat_float: [[f64; 64]; 64]) -> usize {
    const EPS: f64 = 1e-9;
    let row_selected = [false; 64];
    let mut i = 0;
    while i < 64 {
        for j in 0 .. 64 {
            if !row_selected[j] && mat_float[j][i] > EPS {
                break;
            }
        }
        i += 1;
    }
    0
}
