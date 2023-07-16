rust
let _ = f; // just leave off the lifetime specifier
let _: for<'a> fn() = f; // With type ascription
