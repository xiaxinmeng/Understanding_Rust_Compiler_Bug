plain
   Compiling rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
[RUSTC-TIMING] sharded_slab test:false 1.374
[RUSTC-TIMING] itertools test:false 1.393
[RUSTC-TIMING] annotate_snippets test:false 2.217
[RUSTC-TIMING] unic_emoji_char test:false 0.451
[RUSTC-TIMING] ryu test:false 0.383
   Compiling polonius-engine v0.13.0
   Compiling tracing-log v0.1.2
[RUSTC-TIMING] crc32fast test:false 0.327
---
[RUSTC-TIMING] bitflags test:false 0.050
[RUSTC-TIMING] build_script_build test:false 0.251
[RUSTC-TIMING] build_script_build test:false 0.300
[RUSTC-TIMING] log test:false 0.740
[RUSTC-TIMING] unic_emoji_char test:false 0.898
[RUSTC-TIMING] sharded_slab test:false 2.299
   Compiling stacker v0.1.14
[RUSTC-TIMING] ryu test:false 0.963
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
---
test test::test_workspaces_cwd ... ok

failures:

---- doc::output_not_captured stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
thread 'doc::output_not_captured' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
[..]☃

did not find in output:
did not find in output:
 Documenting a v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t780/foo/a)
    Checking a v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t780/foo/a)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t780/foo)
', src/tools/cargo/tests/testsuite/doc.rs:817:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


---
expected success, got: exit status: 101


Build completed unsuccessfully in 0:20:55
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
