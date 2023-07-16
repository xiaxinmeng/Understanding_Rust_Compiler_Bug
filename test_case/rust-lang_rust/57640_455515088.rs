
trait std::ops::Co<T> {
    extern "rust-await" fn await(self) -> T;
}

impl<T> Co<T> for Future<Output=T> { }
