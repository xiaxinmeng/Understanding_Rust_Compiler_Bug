rust
pub fn min_array_ok() -> [i128; 1] {
    [i128::MIN]
}

pub fn min_array_nok() -> [i128; 1] {
    [i128::MIN; 1]
}
