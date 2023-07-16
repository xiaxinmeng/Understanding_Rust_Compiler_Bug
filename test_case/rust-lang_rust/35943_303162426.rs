rust
impl Error+'static {
    fn downcast_cause<T: Error+'static>(&self) -> Option<&T> {
        self.cause()
            .map(|err| { mem::transmute::<&Error, &(Error+'static)>(err) })
            .and_then( |err| err.downcast_ref::<T>() )
    }
}
