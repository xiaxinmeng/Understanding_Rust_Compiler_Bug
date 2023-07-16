plain
[00:01:32] configure: 
[00:01:32] configure: rust.dist-src        := False
[00:01:32] configure: build.extended       := True
[00:01:32] configure: build.print-step-timings := True
[00:01:32] configure: rust.remap-debuginfo := True
[00:01:32] configure: build.submodules     := False
[00:01:32] configure: build.docs           := False
[00:01:32] configure: llvm.static-libstdcpp := True
[00:01:32] configure: build.locked-deps    := True
---
[00:46:20] ---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
[00:46:20] diff of stderr:
[00:46:20] 
[00:46:20] 2    |
[00:46:20] 3 note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...
[00:46:20] 4 note: ...which requires const-evaluating `Foo::bytes::{{constant}}`...
[00:46:20] -   --> $SRC_DIR/libcore/mem.rs:LL:COL
[00:46:20] -    |
[00:46:20] - LL |     intrinsics::size_of::<T>()
[00:46:20] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:46:20] 9    = note: ...which again requires computing layout of `Foo`, completing the cycle
[00:46:20] 10 note: cycle used when const-evaluating `Foo::bytes::{{constant}}`
[00:46:20] -   --> $SRC_DIR/libcore/mem.rs:LL:COL
[00:46:20] -    |
[00:46:20] - LL |     intrinsics::size_of::<T>()
[00:46:20] 15 
[00:46:20] 16 error: aborting due to previous error
[00:46:20] 17 
[00:46:20] 
[00:46:20] 
[00:46:20] 
[00:46:20] The actual stderr differed from the expected stderr.
[00:46:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:46:20] To update references, rerun the tests and pass the `--bless` flag
[00:46:20] To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
[00:46:20] error: 1 errors occurred comparing output.
[00:46:20] status: exit code: 1
[00:46:20] status: exit code: 1
[00:46:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary" "-A" "unused"
[00:46:20] ------------------------------------------
[00:46:20] 
[00:46:20] ------------------------------------------
[00:46:20] stderr:
[00:46:20] stderr:
[00:46:20] ------------------------------------------
[00:46:20] {"message":"cycle detected when computing layout of `Foo`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n