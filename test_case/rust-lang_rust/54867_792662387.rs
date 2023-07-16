rust
pub fn reversed(mut n: u128) -> u128 { // In base 10.
    let mut reversed = 0;
    while n != 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    reversed
}
