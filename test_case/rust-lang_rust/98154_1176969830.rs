plain
   Compiling hashbrown v0.12.0
   Compiling object v0.26.2
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.16.0
error: an `#[unstable]` annotation here has no effect
    |
    |
105 | #[unstable(feature = "empty_write", issue = "none")]
    |
    |
    = note: `#[deny(ineffective_unstable_trait_impl)]` on by default


error: an `#[unstable]` annotation here has no effect
    |
    |
129 | #[unstable(feature = "empty_write", issue = "none")]
    |
    = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information

error: could not compile `std` due to 2 previous errors
