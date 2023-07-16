rust
match a {
    A::A => 1,
    #[cfg(feature = "feature_b")] A::B => 0,
    #[cfg(feature = "feature_c")] A::C => 0,
}
