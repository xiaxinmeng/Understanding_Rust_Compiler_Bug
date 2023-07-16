rust
struct Decimal<const X: u32>(/* fields unimportant */);

impl<const X: u32, const Y: u32> From<Decimal<X>> for Decimal<Y> {
    fn from(previous: Decimal<X>) -> Self {
        /*
        mathematical conversions
        */
        Self(/*ommited*/)
    }
}
