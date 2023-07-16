rust
pub fn zero(d: &mut [Vec<i32>]) {
    let n = d.len();
    for i in 0 .. n {
        let row = &mut d[i][.. n];
        for j in 0 .. n {
            row[j] = 0;
        }
    }
}
