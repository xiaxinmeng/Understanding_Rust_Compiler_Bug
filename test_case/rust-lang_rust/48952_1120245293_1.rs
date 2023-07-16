
the trait bound `[closure@src\constants.rs:4:81: 4:89]: ~const FnOnce<()>` is not satisfied
the trait `~const FnOnce<()>` is not implemented for `[closure@src\constants.rs:4:81: 4:89]`
wrap the `[closure@src\constants.rs:4:81: 4:89]` in a closure with no arguments: `|| { /* code */ }`rustcE0277
constants.rs(4, 66): the trait `FnOnce<()>` is implemented for `[closure@src\constants.rs:4:81: 4:89]`, but that implementation is not `const`
option.rs(797, 12): required by a bound in `Option::<T>::unwrap_or_else`
