rust
struct Wrapper<T>(T);

impl<T> fmt::Debug for Wrapper<T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}
