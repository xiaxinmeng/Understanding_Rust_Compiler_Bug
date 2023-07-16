rust
// suppose we decide to const-ify Path::new
impl Path {
    #[stable(feature = "rust1", since = "1.0.0")]
    #[const_unstable(feature = "const_path_new")]
    pub const fn new<S: AsRef<OsStr> + ?Sized>(s: &S) -> &Path {
        /* ... */
    }
}

// existing unstable const fn
#[unstable(feature = "unique", issue = "27730")]
impl<T: ?Sized> Unique<T> {
    #[const_unstable(feature = "const_unique_new")]
    pub const unsafe fn new(ptr: *mut T) -> Unique<T> {
        /* ... */
    }
}
