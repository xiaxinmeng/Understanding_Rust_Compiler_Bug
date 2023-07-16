rust
    // this is the "closure" version of DerefMove. The alternative would be to have an associated type
    // `Cleanup` and return `(Self::Target, Self::Cleanup)`, but that wouldn't work with unsized
    // rvalues because you can't return a DST by value
    fn deref_move<F: FnOnce(Self::Target) -> O, O>(self, f: F) -> O;

    // explicit form
    fn deref_move<F: for<'a>FnOnce(&'a move Self::Target) -> O, O>(&'a move self, f: F) -> O;
    