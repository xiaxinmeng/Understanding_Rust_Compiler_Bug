rust
/// A slice of ordered values, with bisect lookup.
/// Note: builtin unsizing by choosing `U = [T; N]`.
pub struct OrdSlice<T, U: ?Sized = [T]>(PhantomData<T>, U);

mod netstack {
    pub struct Runtime<'net> {
        _foo: &'net mut OrdSlice<Addr>,
    }
}

mod embedded_app {
    pub struct FindFoo<'net> {
        target: &'net RefCell<OrdSlice<Addr>>,
    }

    enum Network<'net> {
        Setup {
            target: &'net RefCell<OrdSlice<Addr>>,
            with_foo_service: Option<FindFoo<'net>>,
        … },
        Running {
            net: Runtime<'net>,
        … },
    }
}

