
---- [ui] ui/impl-trait/example-calendar.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6
command: "/home/david/Projects/rust/rust7/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/home/david/Projects/rust/rust7/src/test/ui/impl-trait/example-calendar.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/home/david/Projects/rust/rust7/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/david/Projects/rust/rust7/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/david/Projects/rust/rust7/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
rustc: /home/david/Projects/rust/rust7/src/llvm-project/llvm/include/llvm/IR/Instructions.h:1181: void llvm::ICmpInst::AssertOK(): Assertion `getOperand(0)->getType() == getOperand(1)->getType() && "Both operands to ICmp instruction are not of the same type!"' failed.

------------------------------------------



failures:
    [ui] ui/impl-trait/example-calendar.rs
