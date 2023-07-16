rust
trait Ice{}
impl<T> Ice for T where [(); 2 * std::mem::size_of::<*mut T>()]:{}
