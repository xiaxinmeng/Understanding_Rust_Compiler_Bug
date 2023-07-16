rust
struct Wrapper<T>(T);

#[delegate(self.0: T)]
impl<T> fmt::Debug for Wrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
