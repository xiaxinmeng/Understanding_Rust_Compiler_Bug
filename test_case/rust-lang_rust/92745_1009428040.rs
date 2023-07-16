
---- [rustdoc] rustdoc/async-move-doctest.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/Users/user/repos/rust/build/x86_64-apple-darwin/stage1/bin/rustdoc" "-L" "/Users/user/repos/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/user/repos/rust/build/x86_64-apple-darwin/test/rustdoc/async-move-doctest/auxiliary" "-o" "/Users/user/repos/rust/build/x86_64-apple-darwin/test/rustdoc/async-move-doctest" "--deny" "warnings" "/Users/user/repos/rust/src/test/rustdoc/async-move-doctest.rs" "--test" "--edition=2018"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'SESSION_GLOBALS should never be overwritten! Use another thread if you need another SessionGlobals', /Users/user/repos/rust/compiler/rustc_span/src/lib.rs:103:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack

------------------------------------------
