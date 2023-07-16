rust
let a = thing.fold(0.0, |a, b| a + b); // Add everything together, starting with 0.0.
let b = thing.fold_first(|a, b| a + b); // It's called _first because we are *not* specifying the first/inital value?
