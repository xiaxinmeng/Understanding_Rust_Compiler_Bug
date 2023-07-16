
[00:42:05] ---- [ui] ui/deprecated-macro_escape-inner.rs stdout ----
[00:42:05] 	ui: /checkout/src/test/ui/deprecated-macro_escape-inner.rs
[00:42:05] thread '[ui] ui/deprecated-macro_escape-inner.rs' panicked at 'failed to exec `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/remote-test-client`: Error { repr: Os { code: 2, message: "No such file or directory" } }', /checkout/src/libcore/result.rs:860:4
[00:42:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:42:05] 
...
[00:42:05] 
[00:42:05] failures:
[00:42:05]     [ui] ui/deprecated-macro_escape-inner.rs
[00:42:05]     [ui] ui/deprecated-macro_escape.rs
[00:42:05]     [ui] ui/deriving-meta-empty-trait-list.rs
[00:42:05]     [ui] ui/enum-size-variance.rs
[00:42:05]     [ui] ui/issue-19100.rs
[00:42:05]     [ui] ui/path-lookahead.rs
[00:42:05]     [ui] ui/test-should-panic-attr.rs
[00:42:05] 
[00:42:05] test result: FAILED. 348 passed; 7 failed; 1 ignored; 0 measured; 0 filtered out
