rust
fn main() {
    let pis_in_180_below = 57.29577951308232087679815_f64;
    let pis_in_180_above = 57.29577951308232087679816_f64;
    // pis_in_180_below < pis_in_180 < pis_in_180_above (in terms of their literals)
    assert_eq!(pis_in_180_below, pis_in_180_above);
}
