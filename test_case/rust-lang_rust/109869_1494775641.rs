rust
trait Empty<T> {}

impl<T> Default for dyn Empty<T>
where
    Self: Sized,
{
    fn default() -> Self {
        ()
    }
}
