
error: implementation of `std::ops::FnOnce` is not general enough
   --> fil3.rs:6:5
    |
6   |       do_op(path, "remove file", std::fs::remove_file); // redundant closure
    |       ^^^^^ doesn't satisfy where-clause
    |
   ::: /Users/ekuber/workspace/rust/library/core/src/ops/function.rs:219:1
    |
219 | / pub trait FnOnce<Args> {
220 | |     /// The returned type after the call operator is used.
221 | |     #[lang = "fn_once_output"]
222 | |     #[stable(feature = "fn_once_output", since = "1.12.0")]
...   |
227 | |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
228 | | }
    | |_- trait `std::ops::FnOnce` defined here
    |
note: due to this where-clause on `do_op`...
   --> fil3.rs:11:24
    |
11  |     F: FnMut(&Path) -> io::Result<()>,
    |                        ^^^^^^^^^^^^^^
    = note: ...`std::ops::FnOnce<(&'1 std::path::Path,)>` would have to be implemented for the type `fn(&std::path::Path) -> std::result::Result<(), std::io::Error> {std::fs::remove_file::<&std::path::Path>}`, for any lifetime `'1`...
    = note: ...but `std::ops::FnOnce<(&'2 std::path::Path,)>` is actually implemented for the type `fn(&'2 std::path::Path) -> std::result::Result<(), std::io::Error> {std::fs::remove_file::<&'2 std::path::Path>}`, for some specific lifetime `'2`
