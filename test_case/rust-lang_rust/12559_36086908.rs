 rust
trait Decorator<T>
{
    fn get_ref<'l>(&'l self) -> &'l T;
}

impl<T: Container, D: Decorator<T>> Container for D
{
    fn len(&self) -> uint
    {
        self.get_ref().len()
    }
}
