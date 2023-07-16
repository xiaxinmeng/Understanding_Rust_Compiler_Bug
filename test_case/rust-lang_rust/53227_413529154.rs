plain
[00:50:43] ....................................................................................................
[00:50:47] ....................................................................................................
[00:50:50] ....................................................................................................
[00:50:53] ........i...........................................................................................
[00:50:58] ..................................................................................F...........i.....
[00:51:01] ..i.................................................................................................
[00:51:07] ....................................................................................................
[00:51:09] ....................................................................................................
[00:51:12] ....................................................................................................
[00:51:14] ....................................................................................................
---
[00:52:43] ---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
[00:52:43] diff of stderr:
[00:52:43] 
[00:52:43] 2    |
[00:52:43] 3 note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...
[00:52:43] 4 note: ...which requires const-evaluating `Foo::bytes::{{constant}}`...
[00:52:43] -   --> $SRC_DIR/libcore/mem.rs:323:14
[00:52:43] +   --> $SRC_DIR/libcore/mem.rs:321:14
[00:52:43] 6    |
[00:52:43] 7 LL |     unsafe { intrinsics::size_of::<T>() }
[00:52:43] 
[00:52:43] 
[00:52:43] 9    = note: ...which again requires computing layout of `Foo`, completing the cycle
[00:52:43] 10 note: cycle used when const-evaluating `Foo::bytes::{{constant}}`
[00:52:43] -   --> $SRC_DIR/libcore/mem.rs:323:14
[00:52:43] +   --> $SRC_DIR/libcore/mem.rs:321:14
[00:52:43] 12    |
[00:52:43] 13 LL |     unsafe { intrinsics::size_of::<T>() }
[00:52:43] 
[00:52:43] 
[00:52:43] The actual stderr differed from the expected stderr.
[00:52:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:52:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:52:43] To update references, rerun the tests and pass the `--bless` flag
[00:52:43] To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
[00:52:43] error: 1 errors occurred comparing output.
[00:52:43] status: exit code: 1
[00:52:43] status: exit code: 1
[00:52:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary" "-A" "unused"
[00:52:43] ------------------------------------------
[00:52:43] 
[00:52:43] ------------------------------------------
[00:52:43] stderr:
[00:52:43] stderr:
[00:52:43] ------------------------------------------
[00:52:43] {"message":"cycle detected when computing layout of `Foo`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n