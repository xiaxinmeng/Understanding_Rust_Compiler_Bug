rust
impl BitOr<NonZeroU8> for NonZeroU8 {         // NonZeroU8 | NonZeroU8
    type Output = NonZeroU8;
}

impl BitOr<u8> for NonZeroU8 {                // NonZeroU8 | u8
    type Output = NonZeroU8;
}

impl BitOr<NonZeroU8> for u8 {                // u8 | NonZeroU8
    type Output = NonZeroU8;
}

impl BitOrAssign<NonZeroU8> for NonZeroU8 {}  // NonZeroU8 |= NonZeroU8
impl BitOrAssign<u8> for NonZeroU8 {}         // NonZeroU8 |= u8
