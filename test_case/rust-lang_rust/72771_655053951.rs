rust
struct Private;

pub struct Public {
    /// A private field with a [`Private`] type.
    private: Private,
}
