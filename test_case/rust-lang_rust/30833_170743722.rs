 rust
impl Deref for Foobar {
    type Target = DoesStuff;

    fn deref(&self) -> &DoesStuff {
        self as &DoesStuff
    }
}
