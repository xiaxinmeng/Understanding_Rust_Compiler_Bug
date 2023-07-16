
---- [ui] ui/print-fuel/print-fuel.rs stdout ----

error: ui test compiled successfully!
status: exit code: 0
command: "/home/wesley/code/rust/rust3/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/wesley/code/rust/rust3/src/test/ui/print-fuel/print-fuel.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/home/wesley/code/rust/rust3/build/x86_64-unknown-linux-gnu/test/ui/print-fuel/print-fuel/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/home/wesley/code/rust/rust3/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-fuel=foo" "-L" "/home/wesley/code/rust/rust3/build/x86_64-unknown-linux-gnu/test/ui/print-fuel/print-fuel/auxiliary" "-A" "unused"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
Fuel used by foo: 3

------------------------------------------

thread '[ui] ui/print-fuel/print-fuel.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [ui] ui/print-fuel/print-fuel.rs

