plain
2019-10-18T16:55:54.5407742Z test [ui] ui/consts/miri_unleashed/mutable_references.rs ... ok
2019-10-18T16:55:54.5453914Z test [ui] ui/consts/miri_unleashed/mutable_references_ice.rs ... ok
2019-10-18T16:55:54.6572304Z test [ui] ui/consts/non-scalar-cast.rs ... ok
2019-10-18T16:55:54.6663274Z test [ui] ui/consts/mozjs-error.rs ... ok
2019-10-18T16:55:54.7381133Z test [ui] ui/consts/offset_from_ub.rs ... FAILED
2019-10-18T16:55:54.8280007Z test [ui] ui/consts/offset_from.rs ... ok
2019-10-18T16:55:54.9945705Z test [ui] ui/consts/partial_qualif.rs ... ok
2019-10-18T16:55:55.0425781Z test [ui] ui/consts/packed_pattern2.rs ... ok
2019-10-18T16:55:55.0471716Z test [ui] ui/consts/projection_qualif.rs ... ok
2019-10-18T16:55:55.0902455Z test [ui] ui/consts/promote_const_let.rs ... ok
---
2019-10-18T17:06:18.7085702Z test [ui] ui/wrapping-int-combinations.rs ... ok
2019-10-18T17:06:18.7085863Z 
2019-10-18T17:06:18.7085959Z failures:
2019-10-18T17:06:18.7123002Z 
2019-10-18T17:06:18.7141725Z ---- [ui] ui/consts/offset_from_ub.rs stdout ----
2019-10-18T17:06:18.7141923Z 
2019-10-18T17:06:18.7142002Z 1 error: any use of this value will cause an error
2019-10-18T17:06:18.7142002Z 1 error: any use of this value will cause an error
2019-10-18T17:06:18.7142318Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-10-18T17:06:18.7142541Z -    |
2019-10-18T17:06:18.7143036Z - LL |           intrinsics::ptr_offset_from(self, origin)
2019-10-18T17:06:18.7143557Z -    |           |
2019-10-18T17:06:18.7143869Z -    |           ptr_offset_from cannot compute offset of pointers into different allocations.
2019-10-18T17:06:18.7143869Z -    |           ptr_offset_from cannot compute offset of pointers into different allocations.
2019-10-18T17:06:18.7144222Z -    |           inside call to `std::ptr::<impl *const Struct>::offset_from` at $DIR/offset_from_ub.rs:17:27
2019-10-18T17:06:18.7144727Z 9    | 
2019-10-18T17:06:18.7144785Z 10   ::: $DIR/offset_from_ub.rs:11:1
2019-10-18T17:06:18.7144874Z 
2019-10-18T17:06:18.7144925Z 21    = note: `#[deny(const_err)]` on by default
2019-10-18T17:06:18.7144988Z 22 
2019-10-18T17:06:18.7145047Z 23 error: any use of this value will cause an error
2019-10-18T17:06:18.7145047Z 23 error: any use of this value will cause an error
2019-10-18T17:06:18.7145287Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-10-18T17:06:18.7145441Z -    |
2019-10-18T17:06:18.7145636Z - LL |           intrinsics::ptr_offset_from(self, origin)
2019-10-18T17:06:18.7145995Z -    |           |
2019-10-18T17:06:18.7146200Z -    |           a memory access tried to interpret some bytes as a pointer
2019-10-18T17:06:18.7146200Z -    |           a memory access tried to interpret some bytes as a pointer
2019-10-18T17:06:18.7146447Z -    |           inside call to `std::ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:23:14
2019-10-18T17:06:18.7146522Z 31    | 
2019-10-18T17:06:18.7146568Z 32   ::: $DIR/offset_from_ub.rs:21:1
2019-10-18T17:06:18.7146660Z 
2019-10-18T17:06:18.7146820Z 38    | |__-
2019-10-18T17:06:18.7146869Z 39 
2019-10-18T17:06:18.7146928Z 40 error: any use of this value will cause an error
2019-10-18T17:06:18.7146928Z 40 error: any use of this value will cause an error
2019-10-18T17:06:18.7147107Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-10-18T17:06:18.7147273Z -    |
2019-10-18T17:06:18.7147457Z - LL |           intrinsics::ptr_offset_from(self, origin)
2019-10-18T17:06:18.7147828Z -    |           |
2019-10-18T17:06:18.7147828Z -    |           |
2019-10-18T17:06:18.7148028Z -    |           exact_div: 1 cannot be divided by 2 without remainder
2019-10-18T17:06:18.7148455Z -    |           inside call to `std::ptr::<impl *const u16>::offset_from` at $DIR/offset_from_ub.rs:31:27
2019-10-18T17:06:18.7148529Z 48    | 
2019-10-18T17:06:18.7148585Z 49   ::: $DIR/offset_from_ub.rs:26:1
2019-10-18T17:06:18.7148674Z 
2019-10-18T17:06:18.7148710Z 
2019-10-18T17:06:18.7148764Z The actual stderr differed from the expected stderr.
2019-10-18T17:06:18.7148764Z The actual stderr differed from the expected stderr.
2019-10-18T17:06:18.7149048Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/offset_from_ub.stderr
2019-10-18T17:06:18.7149286Z To update references, rerun the tests and pass the `--bless` flag
2019-10-18T17:06:18.7149616Z To only update this specific test, also pass `--test-args consts/offset_from_ub.rs`
2019-10-18T17:06:18.7149737Z error: 1 errors occurred comparing output.
2019-10-18T17:06:18.7149799Z status: exit code: 1
2019-10-18T17:06:18.7149799Z status: exit code: 1
2019-10-18T17:06:18.7150699Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary" "-A" "unused"
2019-10-18T17:06:18.7152005Z ------------------------------------------
2019-10-18T17:06:18.7152061Z 
2019-10-18T17:06:18.7152312Z ------------------------------------------
2019-10-18T17:06:18.7152388Z stderr:
2019-10-18T17:06:18.7152388Z stderr:
2019-10-18T17:06:18.7152626Z ------------------------------------------
2019-10-18T17:06:18.7152709Z error: any use of this value will cause an error
2019-10-18T17:06:18.7152789Z    | 
2019-10-18T17:06:18.7152866Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:11:1
2019-10-18T17:06:18.7152945Z    |
2019-10-18T17:06:18.7153021Z LL | / pub const DIFFERENT_ALLOC: usize = {
2019-10-18T17:06:18.7153093Z LL | |     //~^ NOTE
2019-10-18T17:06:18.7153179Z LL | |     let uninit = std::mem::MaybeUninit::<Struct>::uninit();
2019-10-18T17:06:18.7153277Z LL | |     let base_ptr: *const Struct = &uninit as *const _ as *const Struct;
2019-10-18T17:06:18.7153549Z LL | |     offset as usize
2019-10-18T17:06:18.7153618Z LL | | };
2019-10-18T17:06:18.7153863Z    | |__-
2019-10-18T17:06:18.7153926Z    |
2019-10-18T17:06:18.7153926Z    |
2019-10-18T17:06:18.7154006Z    = note: `#[deny(const_err)]` on by default
2019-10-18T17:06:18.7154056Z 
2019-10-18T17:06:18.7154140Z error: any use of this value will cause an error
2019-10-18T17:06:18.7154215Z    | 
2019-10-18T17:06:18.7154298Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:21:1
2019-10-18T17:06:18.7154373Z    |
2019-10-18T17:06:18.7154606Z LL | / pub const NOT_PTR: usize = {
2019-10-18T17:06:18.7154858Z LL | |     //~^ NOTE
2019-10-18T17:06:18.7154924Z LL | |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }
2019-10-18T17:06:18.7155157Z    | |__-
2019-10-18T17:06:18.7155186Z 
2019-10-18T17:06:18.7155246Z error: any use of this value will cause an error
2019-10-18T17:06:18.7155300Z    | 
2019-10-18T17:06:18.7155300Z    | 
2019-10-18T17:06:18.7155365Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:26:1
2019-10-18T17:06:18.7155421Z    |
2019-10-18T17:06:18.7155487Z LL | / pub const NOT_MULTIPLE_OF_SIZE: usize = {
2019-10-18T17:06:18.7155548Z LL | |     //~^ NOTE
2019-10-18T17:06:18.7155599Z LL | |     let data = [5u8, 6, 7];
2019-10-18T17:06:18.7155665Z LL | |     let base_ptr = data.as_ptr();
2019-10-18T17:06:18.7155769Z LL | |     offset as usize
2019-10-18T17:06:18.7155820Z LL | | };
2019-10-18T17:06:18.7155988Z    | |__-
2019-10-18T17:06:18.7156018Z 
---
2019-10-18T17:06:18.7156351Z 
2019-10-18T17:06:18.7156378Z 
2019-10-18T17:06:18.7156410Z 
2019-10-18T17:06:18.7156454Z failures:
2019-10-18T17:06:18.7156511Z     [ui] ui/consts/offset_from_ub.rs
2019-10-18T17:06:18.7156789Z test result: FAILED. 9140 passed; 1 failed; 59 ignored; 0 measured; 0 filtered out
2019-10-18T17:06:18.7156850Z 
2019-10-18T17:06:18.7163952Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-18T17:06:18.7164391Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-18T17:06:18.7164391Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-18T17:06:18.7179372Z 
2019-10-18T17:06:18.7180025Z 
2019-10-18T17:06:18.7184548Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-18T17:06:18.7185085Z 
2019-10-18T17:06:18.7185114Z 
2019-10-18T17:06:18.7192363Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-10-18T17:06:18.7192667Z Build completed unsuccessfully in 1:10:36
2019-10-18T17:06:18.7192667Z Build completed unsuccessfully in 1:10:36
2019-10-18T17:06:18.7236901Z == clock drift check ==
2019-10-18T17:06:18.7248991Z   local time: Fri Oct 18 17:06:18 UTC 2019
2019-10-18T17:06:19.0046231Z   network time: Fri, 18 Oct 2019 17:06:18 GMT
2019-10-18T17:06:19.0049305Z == end clock drift check ==
2019-10-18T17:06:20.3872693Z 
2019-10-18T17:06:20.3986849Z ##[error]Bash exited with code '1'.
2019-10-18T17:06:20.4033713Z ##[section]Starting: Upload CPU usage statistics
2019-10-18T17:06:20.4038465Z ==============================================================================
2019-10-18T17:06:20.4038563Z Task         : Bash
2019-10-18T17:06:20.4038624Z Description  : Run a Bash script on macOS, Linux, or Windows
