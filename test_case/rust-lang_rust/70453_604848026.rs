rust
type A<T> = [u8; std::mem::size_of::<T>()]; // ERROR the size for values of type `T` cannot be known at compilation time
