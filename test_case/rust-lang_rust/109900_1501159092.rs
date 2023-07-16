plain
---- [ui] tests/ui/issues/issue-61696.rs stdout ----

error: test run failed!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61696" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61696/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'internal error: entered unreachable code', fake-test-src-base/issues/issue-61696.rs:64:9
------------------------------------------



