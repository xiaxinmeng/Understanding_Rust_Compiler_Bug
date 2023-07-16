
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in function call due to conflicting requirements
  --> src/main.rs:17:13
   |
17 |     let t = into_impl(x);
   |             ^^^^^^^^^^^^
   |
note: first, the lifetime cannot outlive the lifetime `'a` as defined on the function body at 16:20...
  --> src/main.rs:16:20
   |
16 | fn extend_lifetime<'a>(x: &'a str) -> &'static str {
   |                    ^^
note: ...so that reference does not outlive borrowed content
  --> src/main.rs:17:23
   |
17 |     let t = into_impl(x);
   |                       ^
note: but, the lifetime must be valid for the anonymous lifetime #2 defined on the body at 18:12...
  --> src/main.rs:18:12
   |
18 |     helper(|_| t)
   |            ^^^^^
note: ...so that the type `impl StaticDefaultRef+std::convert::AsRef<str>+?Sized` is not borrowed for too long
  --> src/main.rs:18:16
   |
18 |     helper(|_| t)
   |                ^

error: aborting due to previous error
