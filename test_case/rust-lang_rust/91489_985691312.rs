rust
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_unstable(feature = "const_deref", issue = "88955")]
impl<B: ?Sized + ToOwned> const Deref for Cow<'_, B>
where
    B::Owned: ~const Borrow<B>,
