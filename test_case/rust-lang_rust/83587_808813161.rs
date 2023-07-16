plain
.................................................................................................... 9400/11718
.................................................................................................... 9500/11718
............................................................i......i................................ 9600/11718
.................................................................................................... 9700/11718
......iiiiiii..iiiiii.i............................................................................. 9800/11718
.................................................................................................... 10000/11718
.................................................................................................... 10100/11718
.................................................................................................... 10200/11718
.................................................................................................... 10300/11718
---
 finished in 0.409 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.113 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 10.499 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.27s

 finished in 2.344 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Finished release [optimized] target(s) in 9.21s
     Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_target-53be088f48a70e58

running 166 tests
.................................................................F..F...........................F.F. 100/166
.F....F......F.......................................F.......F....

---- spec::tests::i686_unknown_netbsd stdout ----
---- spec::tests::i686_unknown_netbsd stdout ----
thread 'spec::tests::i686_unknown_netbsd' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_target/src/spec/i686_unknown_netbsd.rs:7:52

---- spec::tests::i686_wrs_vxworks stdout ----
---- spec::tests::i686_wrs_vxworks stdout ----
thread 'spec::tests::i686_wrs_vxworks' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_target/src/spec/i686_wrs_vxworks.rs:7:52
---- spec::tests::powerpc64_wrs_vxworks stdout ----
---- spec::tests::powerpc64_wrs_vxworks stdout ----
thread 'spec::tests::powerpc64_wrs_vxworks' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_target/src/spec/powerpc64_wrs_vxworks.rs:7:52
---- spec::tests::powerpc_wrs_vxworks stdout ----
---- spec::tests::powerpc_wrs_vxworks stdout ----
thread 'spec::tests::powerpc_wrs_vxworks' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_target/src/spec/powerpc_wrs_vxworks.rs:6:52
---- spec::tests::powerpc_wrs_vxworks_spe stdout ----
---- spec::tests::powerpc_wrs_vxworks_spe stdout ----
thread 'spec::tests::powerpc_wrs_vxworks_spe' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_target/src/spec/powerpc_wrs_vxworks_spe.rs:6:52
---- spec::tests::powerpc_unknown_netbsd stdout ----
---- spec::tests::powerpc_unknown_netbsd stdout ----
thread 'spec::tests::powerpc_unknown_netbsd' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_target/src/spec/powerpc_unknown_netbsd.rs:6:52
---- spec::tests::sparc64_unknown_netbsd stdout ----
---- spec::tests::sparc64_unknown_netbsd stdout ----
thread 'spec::tests::sparc64_unknown_netbsd' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_target/src/spec/sparc64_unknown_netbsd.rs:7:52
---- spec::tests::x86_64_unknown_netbsd stdout ----
---- spec::tests::x86_64_unknown_netbsd stdout ----
thread 'spec::tests::x86_64_unknown_netbsd' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_target/src/spec/x86_64_unknown_netbsd.rs:7:52
---- spec::tests::x86_64_wrs_vxworks stdout ----
---- spec::tests::x86_64_wrs_vxworks stdout ----
thread 'spec::tests::x86_64_wrs_vxworks' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_target/src/spec/x86_64_wrs_vxworks.rs:7:52

failures:
    spec::tests::i686_unknown_netbsd
    spec::tests::i686_wrs_vxworks
---
    spec::tests::x86_64_wrs_vxworks

test result: FAILED. 157 passed; 9 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass '-p rustc_target --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_target" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:23:41
