rust
impl DerefMut for SomeType {
    fn deref_mut … -> &mut SomePointee {
        impl Deref for SomeType { … }
        …
    }
}

impl Ord for SomeType {
    fn cmp … {
        impl PartialOrd for SomeType {
            … Some(self.cmp(other)) …
        }
        …
    }
}

// And on nightly
impl Fn<(…,)> for SomeType {
    fn call (&self, …) -> Ret {
        impl FnMut…
        impl FnOnce…

        …
    }
}
