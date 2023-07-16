rust
// "Fixed" function
// We can change the original definition to the following:
fn get<T, X>(arg: X) -> T where X: Into<...> {...}

// And use it like that:
let result = get::<i32, _>(...); // note the `_` as the second type parameter
                                 // it is being auto-derived, and everyone is happy!
