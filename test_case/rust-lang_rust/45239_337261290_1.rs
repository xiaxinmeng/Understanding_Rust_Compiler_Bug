rust
#![allow(exceeding_bitshifts, const_err)]

const NUMBER: u64 = 0b_10101010;
const FOO: u64 = NUMBER >> 65;

fn main() {

    fn shift(i: u32) -> u64 {
        println!("foo");
        NUMBER >> i
    }

    // ctfe used to produce 0 instead of shifting by 1
    assert_eq!(shift(65), NUMBER >> 65);
    assert_eq!(NUMBER >> 65, shift(65));
    assert_eq!(shift(65), FOO);
    assert_eq!(FOO, shift(65));
    assert_eq!(shift(65), 0b1010101);
    assert_eq!(NUMBER >> 65, 0b1010101);
    assert_eq!(FOO, 0b1010101);
}
