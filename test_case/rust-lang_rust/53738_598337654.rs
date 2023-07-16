
#[deny(single_use_lifetimes)]

#[derive(Eq)]
struct Foo<'a, T> {
    /// a reference to the underlying secret data that will be derefed
    pub data: &'a mut T,
}
