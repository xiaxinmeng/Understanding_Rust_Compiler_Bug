rust
    fn make_owned_pin<'a, T: 'a + ?Sized>(value: &'a move T) -> PinMove<'a, T> { ... }

    struct Thunk<'a> {
        f: &'a move FnOnce()
    }
    