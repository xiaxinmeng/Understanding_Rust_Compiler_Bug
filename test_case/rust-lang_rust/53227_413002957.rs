plain
[00:49:20] ....................................................................................................
[00:49:23] ....................................................................................................
[00:49:26] ....................................................................................................
[00:49:30] .......i............................................................................................
[00:49:34] .................................................................................F...........i......
[00:49:37] .i..................................................................................................
[00:49:43] ....................................................................................................
[00:49:45] ....................................................................................................
[00:49:48] ....................................................................................................
[00:49:50] ....................................................................................................
---
[00:51:16] ---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
[00:51:16] diff of stderr:
[00:51:16] 
[00:51:16] 2    |
[00:51:16] 3 note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...
[00:51:16] 4 note: ...which requires const-evaluating `Foo::bytes::{{constant}}`...
[00:51:16] -   --> $SRC_DIR/libcore/mem.rs:323:14
[00:51:16] +   --> $SRC_DIR/libcore/mem.rs:321:14
[00:51:16] 6    |
[00:51:16] 7 LL |     unsafe { intrinsics::size_of::<T>() }
[00:51:16] 
[00:51:16] 
[00:51:16] 9    = note: ...which again requires computing layout of `Foo`, completing the cycle
[00:51:16] 10 note: cycle used when const-evaluating `Foo::bytes::{{constant}}`
[00:51:16] -   --> $SRC_DIR/libcore/mem.rs:323:14
[00:51:16] +   --> $SRC_DIR/libcore/mem.rs:321:14
[00:51:16] 12    |
[00:51:16] 13 LL |     unsafe { intrinsics::size_of::<T>() }
[00:51:16] 
[00:51:16] 
[00:51:16] The actual stderr differed from the expected stderr.
[00:51:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:51:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:51:16] To update references, rerun the tests and pass the `--bless` flag
[00:51:16] To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
[00:51:16] error: 1 errors occurred comparing output.
[00:51:16] status: exit code: 1
[00:51:16] status: exit code: 1
[00:51:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary" "-A" "unused"
[00:51:16] ------------------------------------------
[00:51:16] 
[00:51:16] ------------------------------------------
[00:51:16] stderr:
[00:51:16] stderr:
[00:51:16] ------------------------------------------
[00:51:16] {"message":"cycle detected when computing layout of `Foo`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n