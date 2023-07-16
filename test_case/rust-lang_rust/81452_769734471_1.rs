rust
impl process::ExitStatus {
    pub fn success_or<E>(self, err: E) -> Result<(), E> {
        self.success_or_else(|| err)
    }

    pub fn success_or_else<F, E>(self, err: F) -> Result<(), E>
    where
        F: FnOnce() -> E,
    {
        if self.success() {
            Ok(())
        } else {
            Err(err())
        }
    }
}
