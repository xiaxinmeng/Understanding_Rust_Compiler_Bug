rust
#[inline]
fn write_v(val: [u64;30], to: &mut [u64;30]) {
    *to = val;
}

pub fn write_to_array(to: &mut [u64;30]) {
    write_v([55;30], to);
    // This optimizes fine on the other hand:
    // *to = [55;30];
}
