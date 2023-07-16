rust
impl<T> Option<T> {
    // The `&?` would be a new syntax that indicates the function
    // is polymorphic over `self`, `&self`, and `&mut self`
    fn unwrap(&? self) -> &? T {
        // I envision `match` as the language primitive which would make it
        // possible to implement a function with such a signature, by extending
        // the upcoming "match default binding modes" feature to support `&?`.
        match self {
            Some(x) => x,
            None => panic!("unwrap on None value"),
        }
    }

    // The above function would behave as though these three functions existed.
    //
    // I'm including the lifetimes explicitly to show that `unwrap` would have
    // a variable number of lifetime parameters based on how it is called;
    // this is something not yet possible to express in rust, and I imagine
    // it would be a huge barrier to adoption and implementation.
    fn unwrap(self) -> T;
    fn unwrap<'a>(&'a self) -> &'a T;
    fn unwrap<'a>(&'a mut self) -> &'a mut T;

    // `unwrap_or_default` would keep its old signature because it can not
    // be made polymorphic over references
    fn unwrap_or_default(self) -> T where T: Default;
}
