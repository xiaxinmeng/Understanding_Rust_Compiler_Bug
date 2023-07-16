
---- [ui] ui/issues/issue-46519.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/home/anyska/Projects/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/anyska/Projects/rust/src/test/ui/issues/issue-46519.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/home/anyska/Projects/rust/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46519/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/anyska/Projects/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-O" "-L" "/home/anyska/Projects/rust/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46519/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: op.layout.ty.builtin_deref(true).is_none() &&
    !matches!(op . layout . fields, FieldsShape :: Primitive)', src/librustc_mir/interpret/validity.rs:664:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
