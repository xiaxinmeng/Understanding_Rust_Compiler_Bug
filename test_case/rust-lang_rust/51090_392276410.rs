plain
[00:42:57] ........................................................................................i...........
[00:43:03] ..............................................i.....................................................
[00:43:07] ....................................................................................................
[00:43:10] ....................................................................................................
[00:43:13] .....................................................................F..............................
[00:43:16] .................F.F...................................F....................................FF......
[00:43:25] ....................................................................................................
[00:43:29] ....................................................................................................
[00:43:35] ...................................................................................i................
[00:43:40] ............................................................i.......................................
[00:43:40] ............................................................i.......................................
[00:43:44] ....................................................................................................
[00:43:49] ....................................................................................................
[00:43:56] .......................................................................................i............
[00:43:58] .....iiiiiiiii...................................................
[00:43:58] 
[00:43:58] ---- [ui] ui/feature-gate-abi.rs stdout ----
[00:43:58] diff of stderr:
[00:43:58] 
[00:43:58] 
[00:43:58] 38    |
[00:43:58] 39    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:43:58] 40 
[00:43:58] - error[E0658]: PTX ABIs are experimental and subject to change
[00:43:58] + error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:43:58] 42   --> $DIR/feature-gate-abi.rs:24:1
[00:43:58] 43    |
[00:43:58] 44 LL | extern "ptx-kernel" fn f6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:43:58] 102    |
[00:43:58] 102    |
[00:43:58] 103    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:43:58] 104 
[00:43:58] - error[E0658]: PTX ABIs are experimental and subject to change
[00:43:58] + error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:43:58] 106   --> $DIR/feature-gate-abi.rs:35:5
[00:43:58] 107    |
[00:43:58] 108 LL |     extern "ptx-kernel" fn m6(); //~ ERROR PTX ABIs are experimental and subject to change
[00:43:58] 166    |
[00:43:58] 166    |
[00:43:58] 167    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:43:58] 168 
[00:43:58] - error[E0658]: PTX ABIs are experimental and subject to change
[00:43:58] + error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:43:58] 170   --> $DIR/feature-gate-abi.rs:44:5
[00:43:58] 171    |
[00:43:58] 172 LL |     extern "ptx-kernel" fn dm6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:43:58] 230    |
[00:43:58] 230    |
[00:43:58] 231    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:43:58] 232 
[00:43:58] - error[E0658]: PTX ABIs are experimental and subject to change
[00:43:58] + error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:43:58] 234   --> $DIR/feature-gate-abi.rs:58:5
[00:43:58] 235    |
[00:43:58] 236 LL |     extern "ptx-kernel" fn m6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:43:58] 294    |
[00:43:58] 294    |
[00:43:58] 295    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:43:58] 296 
[00:43:58] - error[E0658]: PTX ABIs are experimental and subject to change
[00:43:58] + error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:43:58] 298   --> $DIR/feature-gate-abi.rs:70:5
[00:43:58] 299    |
[00:43:58] 300 LL |     extern "ptx-kernel" fn im6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:43:58] 358    |
[00:43:58] 358    |
[00:43:58] 359    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:43:58] 360 
[00:43:58] - error[E0658]: PTX ABIs are experimental and subject to change
[00:43:58] + error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:43:58] 362   --> $DIR/feature-gate-abi.rs:81:11
[00:43:58] 363    |
[00:43:58] 364 LL | type A6 = extern "ptx-kernel" fn (); //~ ERROR PTX ABIs are experimental and subject to change
[00:43:58] 422    |
[00:43:58] 422    |
[00:43:58] 423    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:43:58] 424 
[00:43:58] - error[E0658]: PTX ABIs are experimental and subject to change
[00:43:58] + error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:43:58] 426   --> $DIR/feature-gate-abi.rs:91:1
[00:43:58] 427    |
[00:43:58] 428 LL | extern "ptx-kernel" {} //~ ERROR PTX ABIs are experimental and subject to change
[00:43:58] 
[00:43:58] The actual stderr differed from the expected stderr.
[00:43:58] The actual stderr differed from the expected stderr.
[00:43:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-abi/feature-gate-abi.stderr
[00:43:58] To update references, rerun the tests and pass the `--bless` flag
[00:43:58] To only update this specific test, also pass `--test-args feature-gate-abi.rs`
[00:43:58] error: 1 errors occurred comparing output.
[00:43:58] status: exit code: 101
[00:43:58] status: exit code: 101
[00:43:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate-abi.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-abi/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-abi/auxiliary" "-A" "unused"
[00:43:58] ------------------------------------------
[00:43:58] 
[00:43:58] ------------------------------------------
[00:43:58] stderr:
[00:43:58] stderr:
[00:43:58] ------------------------------------------
[00:43:58] {"message":"intrinsics are subject to change","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n