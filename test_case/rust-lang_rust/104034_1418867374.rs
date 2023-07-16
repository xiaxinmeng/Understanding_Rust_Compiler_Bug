rust
trait Uninhabited {
    fn uninhabited(self) -> !;
}

impl<T> Uninhabited for Wrapper<T>
where
    T: Uninhabited,
{
    fn uninhabited(self) -> ! {
        let Wrapper(inner) = self;
        match inner {}
    }
}
