plain
.......................................F..........................................

failures:

---- [ui] tests/ui/upstream-llvm/issue-110743-debug-output.rs stdout ----


- 80/7680000120 (0.00%) spills, 7680000040/7680000120 (100.00%) variables
- 80/7680000120 (0.00%) spills, 7680000040/7680000120 (100.00%) variables


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/upstream-llvm/issue-110743-debug-output/issue-110743-debug-output.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args upstream-llvm/issue-110743-debug-output.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/upstream-llvm/issue-110743-debug-output.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/upstream-llvm/issue-110743-debug-output" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/upstream-llvm/issue-110743-debug-output/auxiliary"
stdout: none
stderr: none


failures:
failures:
    [ui] tests/ui/upstream-llvm/issue-110743-debug-output.rs
test result: FAILED. 14733 passed; 1 failed; 132 ignored; 0 measured; 0 filtered out; finished in 130.13s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:11:21
