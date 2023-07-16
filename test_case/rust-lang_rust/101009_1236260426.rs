plain
    |
86  |   macro_rules! assert_ne {
    |   ---------------------- in this expansion of `assert_ne!` (#2)
...
90  |                   if *left_val == *right_val {
    |
   ::: compiler/rustc_interface/src/tests.rs:706:5
    |
706 | /     macro_rules! tracked {
706 | /     macro_rules! tracked {
707 | |         ($name: ident, $non_default_value: expr) => {
708 | |             opts = reference.clone();
709 | |             assert_ne!(opts.unstable_opts.$name, $non_default_value);
...   |
712 | |         };
713 | |     }
    | |_____- in this expansion of `tracked!` (#1)
    | |_____- in this expansion of `tracked!` (#1)
...
777 |       tracked!(profiler_runtime, "abc".to_string());
    |
    = note: expected enum `Option<std::string::String>`
             found struct `std::string::String`
help: try wrapping the expression in `Some`
help: try wrapping the expression in `Some`
    |
90  |                 if *left_val == Some(*right_val) {

error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:777:32
    |
    |
710 |             opts.unstable_opts.$name = $non_default_value;
...
...
777 |     tracked!(profiler_runtime, "abc".to_string());
    |
    = note: expected enum `Option<std::string::String>`
             found struct `std::string::String`
help: try wrapping the expression in `Some`
help: try wrapping the expression in `Some`
    |
777 |     tracked!(profiler_runtime, Some("abc".to_string()));

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_interface` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
