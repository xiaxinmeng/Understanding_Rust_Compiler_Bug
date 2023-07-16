Rust
fn main() {
    let mut x = [(1,2),(2,3),(3,4)];
    let (i, j) = (0, 0);
    let a = &mut x[i].0;
    let b = &mut x[j].1;
    
    std::mem::swap(a, b);
}
