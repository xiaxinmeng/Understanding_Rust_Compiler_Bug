
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 1 test
F
failures:

---- [ui] rustdoc-ui/issue-80992.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/home/omer/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustdoc" "/home/omer/rust/rust/src/test/rustdoc-ui/issue-80992.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/home/omer/rust/rust/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-80992" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/omer/rust/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/home/omer/rust/rust/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-80992/auxiliary"
stdout:
------------------------------------------

running 1 test
test src/test/rustdoc-ui/issue-80992.rs - test (line 7) ... FAILED

failures:

---- src/test/rustdoc-ui/issue-80992.rs - test (line 7) stdout ----
thread 'src/test/rustdoc-ui/issue-80992.rs - test (line 7)' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:974:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

query stack during panic:
end of query stack


failures:
    src/test/rustdoc-ui/issue-80992.rs - test (line 7)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C debuginfo=0


------------------------------------------



failures:
    [ui] rustdoc-ui/issue-80992.rs

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 95 filtered out; finished in 0.04s

Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
