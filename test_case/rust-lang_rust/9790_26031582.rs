 rust
pub use internal::S;
mod internal
{
    pub struct S;
    impl S
    {
        pub fn test(&self) {}
    }
}
