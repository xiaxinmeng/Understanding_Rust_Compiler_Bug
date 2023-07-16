rust
#[test]
#[should_panic(expect = "vec len overflow")]
fn vec_into_flattened_panics_for_capacity_overflow() {
    let n = 1 << (usize::BITS / 2);
    let _ = vec![[(); n]; n].into_flattened();
}
