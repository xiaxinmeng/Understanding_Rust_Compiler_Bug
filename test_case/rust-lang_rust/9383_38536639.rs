 rust
#[feature(macro_rules)];

macro_rules! macro_invoke (($ex:expr) => ({let _x = 9; $ex()}))
pub fn main() {
    let _x = 8;
    assert_eq!(macro_invoke!(|| _x),8)
}
