 rust
pub fn next_power_of_two<T: UnsignedInt>(x: T) -> T {
    let bits = std::mem::size_of::<T>() * 8;
    let one: T = Int::one();
    one << ((bits - (x - one).leading_zeros()) % bits)
}

pub fn checked_next_power_of_two<T: UnsignedInt>(x: T) -> Option<T> {
    let npot = next_power_of_two(x);
    if npot >= x {
        Some(npot)
    } else {
        None
    }
}

#[test]
fn t() {
    assert_eq!(next_power_of_two::<u64>(0), 1);
    assert_eq!(next_power_of_two::<u64>(1), 1);
    assert_eq!(next_power_of_two::<u64>(2), 2);
    assert_eq!(next_power_of_two::<u64>(3), 4);
    assert_eq!(next_power_of_two::<u64>(4), 4);
    assert_eq!(next_power_of_two::<u64>(5), 8);
    assert_eq!(next_power_of_two::<u64>(-1), 1);
    assert_eq!(next_power_of_two::<u64>(-2), 1);
    assert_eq!(next_power_of_two::<u64>(-3), 1);
    assert_eq!(next_power_of_two::<u64>(-4), 1);
    assert_eq!(next_power_of_two::<u64>(-5), 1);
}
