plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0382]: use of moved value: `formats`
   --> compiler/rustc_metadata/src/native_libs.rs:47:33
    |
36  |     let formats = if verbatim.unwrap_or(false) {
    |         ------- move occurs because `formats` has type `Vec<(Cow<'_, str>, Cow<'_, str>)>`, which does not implement the `Copy` trait
...
47  |         for (prefix, suffix) in formats {
    |                                 ^^^^^^^ `formats` moved due to this implicit call to `.into_iter()`, in previous iteration of loop
    |
note: this function takes ownership of the receiver `self`, which moves `formats`
    |
261 |     fn into_iter(self) -> Self::IntoIter;
    |                  ^^^^
    |                  ^^^^
help: consider iterating over a slice of the `Vec<(Cow<'_, str>, Cow<'_, str>)>`'s content to avoid moving into the `for` loop
    |
47  |         for (prefix, suffix) in &formats {

For more information about this error, try `rustc --explain E0382`.
error: could not compile `rustc_metadata` due to previous error
warning: build failed, waiting for other jobs to finish...
