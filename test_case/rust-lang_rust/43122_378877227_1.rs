rust
let mut generator = || {
    if false { yield { return () } as Box<Debug> };
    yield Box::new(123i32);
    yield Box::new("hello");
};
