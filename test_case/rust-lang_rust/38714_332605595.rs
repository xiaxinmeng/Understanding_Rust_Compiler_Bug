
// non-anonymous region appears in `x`, so keep both `x` and `y` just as they wrote it
foo(|x: &'a str, y: &str| ..)

// `_` in the user's signature is just weird, ignore the expected type, which I think we would also do today.
bar(|x: Vec<_>, y| ..)

// signatures not unifiable, just use what they wrote
fn baz<F>(f: F) where F: FnOnce(&str)
baz(|x: &u8| ...)
