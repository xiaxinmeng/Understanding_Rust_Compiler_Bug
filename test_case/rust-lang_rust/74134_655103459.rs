rust
struct Private;

pub struct Public {
    /// A private field with a [`Private`] type.
    pub public: Private,
}
