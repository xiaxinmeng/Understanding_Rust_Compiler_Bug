rust
use std::convert::TryFrom;

fn main() {
    const N: usize = 16;
    type Array = [u8; N];
    let array: Array = [0; N];
    let slice: &[u8] = &array[..];

    let result = <&Array>::try_from(slice);
    assert_eq!(&array, result.unwrap());
}
