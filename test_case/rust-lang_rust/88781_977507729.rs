plain
test lto::cdylib_and_rlib ... ok

failures:

---- doc::output_not_captured stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
thread 'doc::output_not_captured' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
[..]☃

did not find in output:
did not find in output:
 Documenting a v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t792/foo/a)
    Checking a v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t792/foo/a)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t792/foo)
', src/tools/cargo/tests/testsuite/doc.rs:817:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


---
expected success, got: exit status: 101


Build completed unsuccessfully in 0:26:06
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
