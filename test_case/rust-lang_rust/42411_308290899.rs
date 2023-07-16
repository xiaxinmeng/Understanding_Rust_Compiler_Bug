rust
trait Auxiliary {
    type Buffer: Default;
}

// by default, the "buffer" is a `Vec<u8>`
impl<T> Auxiliary for T {
    default type Buffer = Vec<u8>;
}
