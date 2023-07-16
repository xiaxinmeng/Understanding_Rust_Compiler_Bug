rust
#[stable(feature = "demo", since = "0.0.0")]
pub enum E {
    // stable because the item is stable
    A,
    #[unstable(feature = "demo_b", issue = "0")]
    B,
}
