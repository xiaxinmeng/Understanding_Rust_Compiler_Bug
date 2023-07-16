plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:051bda1c
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:54:23]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:54:27]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:56:02]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:56:13]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:56:13] error: `$warn_text:expr` is followed (through repetition) by `$warn_val:expr`, which is not allowed for `expr` fragments
[00:56:13]    --> librustc/session/config.rs:314:74
[00:56:13]     |
[00:56:13] 314 |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
[00:56:13]     |                                                          --------------  ^^^^^^^^^^^^^^^ this fragment is followed by the first fragment in this repetition without a valid separator
[00:56:13]     |                                                          |
[00:56:13]     |                                                          this is the first fragment in the evaluated repetition
[00:56:16] error[E0432]: unresolved import `session::config::Options`
[00:56:16]   --> librustc/util/profiling.rs:11:5
[00:56:16]    |
[00:56:16] 11 | use session::config::Options;
---
[00:56:18] 
[00:56:18] error[E0412]: cannot find type `Options` in this scope
[00:56:18]     --> librustc/session/config.rs:1471:35
[00:56:18]      |
[00:56:18] 1471 | pub fn build_target_config(opts: &Options, sp: &Handler) -> Config {
[00:56:18]      |                                   ^^^^^^^ did you mean `Option`?
[00:56:18]      |
[00:56:18] 14   | use getopts::Options;
[00:56:18]      |
[00:56:18] 14   | use test::Options;
---
[00:56:18] 
[00:56:18] error[E0412]: cannot find type `Options` in module `config`
[00:56:18]   --> librustc/session/mod.rs:66:23
[00:56:18]    |
[00:56:18] 66 |     pub opts: config::Options,
[00:56:18] help: possible candidates are found in other modules, you can import them into scope
[00:56:18]    |
[00:56:18] 11 | use getopts::Options;
[00:56:18]    |
---
[00:56:18] 
[00:56:18] error[E0412]: cannot find type `Options` in module `config`
[00:56:18]    --> librustc/session/mod.rs:889:51
[00:56:18]     |
[00:56:18] 889 |     pub fn query_threads_from_opts(opts: &config::Options) -> usize {
[00:56:18] help: possible candidates are found in other modules, you can import them into scope
[00:56:18]     |
[00:56:18] 11  | use getopts::Options;
[00:56:18]     |
---
[00:56:18] 
[00:56:40] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:56:40]   --> librustc/lint/levels.rs:65:40
[00:56:40]    |
[00:56:40] 65 |         for &(ref lint_name, level) in &sess.opts.lint_opts {
[00:56:40]    |
[00:56:40]    = help: the trait `std::marker::Sized` is not implemented for `str`
[00:56:40]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:56:40]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:56:40]    = note: only the last element of a tuple may have a dynamically sized type
[00:56:42] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:56:42]     --> librustc/mir/mod.rs:2351:33
[00:56:42]      |
[00:56:42]      |
[00:56:42] 2351 |                             let name = if tcx.sess.opts.debugging_opts.span_free_formats {
[00:56:42]      |
[00:56:42]      = help: the trait `std::marker::Sized` is not implemented for `str`
[00:56:42]      = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:56:42]      = note: all local variables must have a statically known size
[00:56:42]      = note: all local variables must have a statically known size
[00:56:42]      = help: unsized locals are gated as an unstable feature
[00:56:42] 
[00:56:43] error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
[00:56:43]    --> librustc/session/mod.rs:700:13
[00:56:43] 700 |             Some(ref sysroot) => sysroot,
[00:56:43] 700 |             Some(ref sysroot) => sysroot,
[00:56:43]     |             ^^^^^^^^^^^^^^^^^ borrow the `Path` instead
[00:56:43]     |
[00:56:43]     = help: within `std::path::Path`, the trait `std::marker::Sized` is not implemented for `[u8]`
[00:56:43]     = note: required because it appears within the type `std::path::Path`
[00:56:43]     = note: required because it appears within the type `std::path::Path`
[00:56:43]     = note: required by `std::prelude::v1::Some`
