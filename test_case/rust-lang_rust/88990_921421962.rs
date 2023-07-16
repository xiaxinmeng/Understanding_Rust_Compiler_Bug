rust
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(1)]
#[rustc_nonnull_optimization_guaranteed]
pub struct NonZero<T: IntegerPrimitive>(T);

// ... then most impl can be freed from the macros ...

// stability compatibility ↓

pub type NonZeroU8 = NonZero<u8>;
...
pub type NonZeroI128 = NonZero<i128>;
