rust
impl PartialOrd for Option<NonZeroU32> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe {
            let a = std::intrinsics::transmute::<&Option<NonZeroU32>, &u32>(self);
            let b = std::intrinsics::transmute::<&Option<NonZeroU32>, &u32>(other);
            a.partial_cmp(b)
        }
    }
}
