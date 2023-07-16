rust
// The align limit was originally smaller (2^15).
// Check that it works with big numbers.
#[repr(align(0x10000))]
struct AlignLarge {
    stuff: [u8; 0x10000],
}

// Check the actual address is aligned
fn is_aligned_to<T>(p: &T, align: usize) -> bool {
    let addr = p as *const T as usize;
    (addr & (align - 1)) == 0
}

fn main() {
    let mut arr = [0; 0x10000];
    arr[0] = 132;
    let large = AlignLarge {
        stuff: arr,
    };
    assert_eq!(large.stuff[0], 132);
    assert_eq!(mem::align_of::<AlignLarge>(), 0x10000);
    assert_eq!(mem::align_of_val(&large), 0x10000);
    assert!(is_aligned_to(&large, 0x10000));
}
