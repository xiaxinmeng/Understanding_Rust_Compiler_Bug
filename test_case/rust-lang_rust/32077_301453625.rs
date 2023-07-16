rust
/// A generic struct.
pub struct GenericStruct<T> {
    i: T
}

impl<T> GenericStruct<T> {
    /// A method on TypedefStruct.
    pub fn generic_impl_method(arg: T) {}
}

/// A typedef for GenericStruct, without any generics.
pub type TypedefStruct = GenericStruct<u8>;

impl TypedefStruct {
    /// A method on TypedefStruct.
    pub fn typedef_impl_method() {}
}
