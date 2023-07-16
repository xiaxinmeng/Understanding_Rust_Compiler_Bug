rust
#[repr(align(correct))]
newtype atomic_u8_t = /*???*/;

#[repr(transparent)]
struct AtomicU8 {
    v: UnsafeCell<atomic_u8_t>
}
