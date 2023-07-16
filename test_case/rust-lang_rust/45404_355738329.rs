rust
default unsafe impl<'a, I, T: 'a> TrustedRandomAccess for Cloned<I>
    where I: TrustedRandomAccess<Item=&'a T>, T: Clone { .. }
