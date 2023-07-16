rust
pub fn main() {
    let tuples = vec![(0u8, 1u8)];
    for (m, n) in &tuples { // (m, n) are bound like &(ref m, ref n)
        let _: &u8 = m;
        println!("{} {}", m, n);
    }
}
