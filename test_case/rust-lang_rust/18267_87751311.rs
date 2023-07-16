
foo.rs:8:20: 8:26 error: internal compiler error: constant expression should not reach expr::trans_def
foo.rs:8     println!("{}", SOME_S.some_stuff);
                            ^~~~~~
note: in expansion of format_args!
<std macros>:2:25: 2:58 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
foo.rs:8:5: 8:39 note: expansion site
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:130
