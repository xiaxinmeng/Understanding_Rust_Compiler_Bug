rust
impl<'a> Origin<'a> {
    /// The root: `'/'`.
    #[doc(hidden)]
    pub const ROOT: Origin<'static> = Origin::const_new("/", None);
}
