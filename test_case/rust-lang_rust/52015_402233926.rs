rust
#[test]
pub fn scalar_test() {
    let z = 0_i128;
    let o = 1_i128;

    if o >> z != o {
        panic!();
    }
}
