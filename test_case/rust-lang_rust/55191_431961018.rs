rust
/// that's some enum, all right
#[unstable(feature="asdf", issue="0")]
pub enum SomeEnum {
    Asdf,
    /// we got some variant here, i guess
    SomeVariant {
        /// it's one field
        one: usize,
        /// it's two field
        two: usize,
    },
    Qwop,
    Girp,
}
