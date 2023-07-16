rust
// "Original" function
// `T` is the type you want to obtain after calling `get`.
// `arg` is some additional value for which you initially use `impl`-type,
//   and you want it to be automatically derived by the compiler.
// Here, the type of `value` is `Into<...>`, but it might be `IntoIterator`, `AsRef`, etc
fn get<T>(arg: impl Into<...>) -> T {...}

// The call side might look like this:
let result = get(...); // this doesn't compile since the compiler cannot derive `T`
let result = get::<i32>(...); // intuitively OK, but for now, we can't do this
                              // (the current issue is exactly about this)
let result: i32 = get(...); // OK, but sometimes it is not possible to specify `T` on the lhs,
                            // because sometimes there is no lhs (e.g. inside another expression)
