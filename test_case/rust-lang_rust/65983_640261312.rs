rust
/// Return a [`rand::Rng`] from [`rand`].
pub fn f() -> impl rand::Rng {
    rand::thread_rng()
}

/// This is outer documentation for [`f`]
pub mod inner {
    /// This is inner documentation for [`f`]
    use super::f;
}

/// This is documentation for the re-export of [`Rng`]
pub use rand::Rng;
