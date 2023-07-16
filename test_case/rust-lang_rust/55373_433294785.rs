plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:142c7d42
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:52:16]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:52:21]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:53:55]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:54:06]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:54:08] error: `$warn_text:expr` is followed (through repetition) by `$warn_val:expr`, which is not allowed for `expr` fragments
[00:54:08]    --> librustc/session/config.rs:314:74
[00:54:08]     |
[00:54:08] 314 |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
[00:54:08]     |                                                          --------------  ^^^^^^^^^^^^^^^ this fragment is followed by the first fragment in this repetition without a valid separator
[00:54:08]     |                                                          |
[00:54:08]     |                                                          this is the first fragment in the evaluated repetition
[00:54:11] error[E0432]: unresolved import `session::config::Options`
[00:54:11]   --> librustc/util/profiling.rs:11:5
[00:54:11]    |
[00:54:11] 11 | use session::config::Options;
---
[00:54:14] 
[00:54:14] error[E0412]: cannot find type `Options` in this scope
[00:54:14]     --> librustc/session/config.rs:1471:35
[00:54:14]      |
[00:54:14] 1471 | pub fn build_target_config(opts: &Options, sp: &Handler) -> Config {
[00:54:14]      |                                   ^^^^^^^ did you mean `Option`?
[00:54:14]      |
[00:54:14] 14   | use getopts::Options;
[00:54:14]      |
[00:54:14] 14   | use test::Options;
---
[00:54:14] 
[00:54:14] error[E0412]: cannot find type `Options` in module `config`
[00:54:14]   --> librustc/session/mod.rs:66:23
[00:54:14]    |
[00:54:14] 66 |     pub opts: config::Options,
[00:54:14] help: possible candidates are found in other modules, you can import them into scope
[00:54:14]    |
[00:54:14] 11 | use getopts::Options;
[00:54:14]    |
---
[00:54:14] 
[00:54:14] error[E0412]: cannot find type `Options` in module `config`
[00:54:14]    --> librustc/session/mod.rs:889:51
[00:54:14]     |
[00:54:14] 889 |     pub fn query_threads_from_opts(opts: &config::Options) -> usize {
[00:54:14] help: possible candidates are found in other modules, you can import them into scope
[00:54:14]     |
[00:54:14] 11  | use getopts::Options;
[00:54:14]     |
---
[00:54:14] 
[00:54:33] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:54:33]   --> librustc/lint/levels.rs:65:40
[00:54:33]    |
[00:54:33] 65 |         for &(ref lint_name, level) in &sess.opts.lint_opts {
[00:54:33]    |
[00:54:33]    = help: the trait `std::marker::Sized` is not implemented for `str`
[00:54:33]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:54:33]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:54:33]    = note: only the last element of a tuple may have a dynamically sized type
[00:54:36] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:54:36]     --> librustc/mir/mod.rs:2351:33
[00:54:36]      |
[00:54:36]      |
[00:54:36] 2351 |                             let name = if tcx.sess.opts.debugging_opts.span_free_formats {
[00:54:36]      |
[00:54:36]      = help: the trait `std::marker::Sized` is not implemented for `str`
[00:54:36]      = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:54:36]      = note: all local variables must have a statically known size
[00:54:36]      = note: all local variables must have a statically known size
[00:54:36]      = help: unsized locals are gated as an unstable feature
[00:54:36] 
[00:54:37] error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
[00:54:37]    --> librustc/session/mod.rs:700:13
[00:54:37] 700 |             Some(ref sysroot) => sysroot,
[00:54:37] 700 |             Some(ref sysroot) => sysroot,
[00:54:37]     |             ^^^^^^^^^^^^^^^^^ borrow the `Path` instead
[00:54:37]     |
[00:54:37]     = help: within `std::path::Path`, the trait `std::marker::Sized` is not implemented for `[u8]`
[00:54:37]     = note: required because it appears within the type `std::path::Path`
[00:54:37]     = note: required because it appears within the type `std::path::Path`
[00:54:37]     = note: required by `std::prelude::v1::Some`
[00:54:45] error: aborting due to 17 previous errors
[00:54:45] 
[00:54:45] Some errors occurred: E0277, E0412, E0422, E0432.
[00:54:45] For more information about an error, try `rustc --explain E0277`.
[00:54:45] For more information about an error, try `rustc --explain E0277`.
[00:54:45] error: Could not compile `rustc`.
[00:54:45] 
[00:54:45] To learn more, run the command again with --verbose.
[00:54:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:54:45] expected success, got: exit code: 101
[00:54:45] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:54:45] travis_fold:end:stage1-rustc

[00:54:45] travis_time:end:stage1-rustc:start=1540532763905769226,finish=1540532966058997712,duration=202153228486

---
travis_time:end:01e95038:start=1540532967460490633,finish=1540532967474365114,duration=13874481
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01a20f20
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0eeb22d4
travis_time:start:0eeb22d4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2c95bedc
$ dmesg | grep -i kill
