rust
pub fn zero(d: &mut [Vec<i32>]) {
    let n = d.len();
    for i in 0..n {
        let di = &mut d[i][..n];
        for j in 0..n {
            di[j] = 0;
        }
    }
}
