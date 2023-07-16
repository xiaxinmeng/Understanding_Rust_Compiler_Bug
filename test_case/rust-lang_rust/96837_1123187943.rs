plain
test [ui] src/test/ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] src/test/ui/process/nofile-limit.rs stdout ----
normalized stderr:
WARN rustc_codegen_ssa::back::link Linker does not support -static-pie command line option. Retrying with -static instead.


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/ui/process/nofile-limit/nofile-limit.stderr
To only update this specific test, also pass `--test-args process/nofile-limit.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/tmp/distcheck/src/test/ui/process/nofile-limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/ui/process/nofile-limit/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ctarget-feature=+crt-static" "-Crpath=no" "-L" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/ui/process/nofile-limit/auxiliary"
stdout: none
--- stderr -------------------------------
WARN rustc_codegen_ssa::back::link Linker does not support -static-pie command line option. Retrying with -static instead.



failures:
failures:
    [ui] src/test/ui/process/nofile-limit.rs

test result: FAILED. 12978 passed; 1 failed; 102 ignored; 0 measured; 0 filtered out; finished in 144.67s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:18:23
make: *** [check] Error 1
Makefile:42: recipe for target 'check' failed
