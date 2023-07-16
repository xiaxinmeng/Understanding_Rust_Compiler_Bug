rust
macro_rules! ty {
    ($ty:ty) => {};
}

ty!(union { field: u32 });
