rust
#[repr(packed)]
struct Foo<T> {
    _filler0: [u8; 1],
    discriminant: u16, /* DeviceKind niche */
    _filler1: [u8; (round_to_multiple(3, align_of::<T>()) - 3) + size_of::<T>()],
}
