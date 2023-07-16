rust
unsafe trait Unsize<U: ?Sized> {
    fn unsize(&self) -> &U
}
