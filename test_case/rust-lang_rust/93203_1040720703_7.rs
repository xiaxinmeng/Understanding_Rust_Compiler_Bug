rust
impl<'env> Scope<'env> {
    pub fn spawn<'scope, F, T>(&'scope self, f: F) -> ScopedJoinHandle<'scope, T>
    where
        F: FnOnce(&Scope<'env>) -> T + Send + 'env,
        T: Send + 'env;                    // ^^^^ here                        
}
