plain
travis_time:end:17d33900:start=1540530678669451371,finish=1540530732059178857,duration=53389727486
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:23:02]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:23:07]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:24:35]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:24:44]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:24:45] error: `$warn_text:expr` is followed (through repetition) by `$warn_val:expr`, which is not allowed for `expr` fragments
[00:24:45]    --> librustc/session/config.rs:314:74
[00:24:45]     |
[00:24:45] 314 |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
[00:24:45]     |                                                          --------------  ^^^^^^^^^^^^^^^ this fragment is followed by the first fragment in this repetition without a valid separator
[00:24:45]     |                                                          |
[00:24:45]     |                                                          this is the first fragment in the evaluated repetition
[00:24:47] error[E0432]: unresolved import `session::config::Options`
[00:24:47]   --> librustc/util/profiling.rs:11:5
[00:24:47]    |
[00:24:47] 11 | use session::config::Options;
---
[00:24:49] 
[00:24:49] error[E0412]: cannot find type `Options` in this scope
[00:24:49]     --> librustc/session/config.rs:1471:35
[00:24:49]      |
[00:24:49] 1471 | pub fn build_target_config(opts: &Options, sp: &Handler) -> Config {
[00:24:49]      |                                   ^^^^^^^ did you mean `Option`?
[00:24:49]      |
[00:24:49] 14   | use getopts::Options;
[00:24:49]      |
[00:24:49] 14   | use test::Options;
---
[00:24:49] 
[00:24:49] error[E0412]: cannot find type `Options` in module `config`
[00:24:49]   --> librustc/session/mod.rs:66:23
[00:24:49]    |
[00:24:49] 66 |     pub opts: config::Options,
[00:24:49] help: possible candidates are found in other modules, you can import them into scope
[00:24:49]    |
[00:24:49] 11 | use getopts::Options;
[00:24:49]    |
---
[00:24:49] 
[00:24:49] error[E0412]: cannot find type `Options` in module `config`
[00:24:49]    --> librustc/session/mod.rs:889:51
[00:24:49]     |
[00:24:49] 889 |     pub fn query_threads_from_opts(opts: &config::Options) -> usize {
[00:24:49] help: possible candidates are found in other modules, you can import them into scope
[00:24:49]     |
[00:24:49] 11  | use getopts::Options;
[00:24:49]     |
---
[00:24:49] 
[00:25:13] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:25:13]   --> librustc/lint/levels.rs:65:40
[00:25:13]    |
[00:25:13] 65 |         for &(ref lint_name, level) in &sess.opts.lint_opts {
[00:25:13]    |
[00:25:13]    = help: the trait `std::marker::Sized` is not implemented for `str`
[00:25:13]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:25:13]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:25:13]    = note: only the last element of a tuple may have a dynamically sized type
[00:25:15] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:25:15]     --> librustc/mir/mod.rs:2351:33
[00:25:15]      |
[00:25:15]      |
[00:25:15] 2351 |                             let name = if tcx.sess.opts.debugging_opts.span_free_formats {
[00:25:15]      |
[00:25:15]      = help: the trait `std::marker::Sized` is not implemented for `str`
[00:25:15]      = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:25:15]      = note: all local variables must have a statically known size
[00:25:15]      = note: all local variables must have a statically known size
[00:25:15]      = help: unsized locals are gated as an unstable feature
[00:25:15] 
[00:25:17] error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
[00:25:17]    --> librustc/session/mod.rs:700:13
[00:25:17] 700 |             Some(ref sysroot) => sysroot,
[00:25:17] 700 |             Some(ref sysroot) => sysroot,
[00:25:17]     |             ^^^^^^^^^^^^^^^^^ borrow the `Path` instead
[00:25:17]     |
[00:25:17]     = help: within `std::path::Path`, the trait `std::marker::Sized` is not implemented for `[u8]`
[00:25:17]     = note: required because it appears within the type `std::path::Path`
[00:25:17]     = note: required because it appears within the type `std::path::Path`
[00:25:17]     = note: required by `std::prelude::v1::Some`
