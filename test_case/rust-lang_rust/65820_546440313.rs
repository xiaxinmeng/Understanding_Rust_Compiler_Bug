plain
2019-10-25T17:13:13.0938506Z test [ui] ui/consts/miri_unleashed/mutable_references.rs ... ok
2019-10-25T17:13:13.1600153Z test [ui] ui/consts/miri_unleashed/non_const_fn.rs ... ok
2019-10-25T17:13:13.2097713Z test [ui] ui/consts/mozjs-error.rs ... ok
2019-10-25T17:13:13.2811106Z test [ui] ui/consts/non-scalar-cast.rs ... ok
2019-10-25T17:13:13.3555280Z test [ui] ui/consts/offset_from_ub.rs ... FAILED
2019-10-25T17:13:13.3806921Z test [ui] ui/consts/offset_from.rs ... ok
2019-10-25T17:13:13.5702259Z test [ui] ui/consts/packed_pattern2.rs ... ok
2019-10-25T17:13:13.6149254Z test [ui] ui/consts/projection_qualif.rs ... ok
2019-10-25T17:13:13.6174714Z test [ui] ui/consts/partial_qualif.rs ... ok
2019-10-25T17:13:13.6614471Z test [ui] ui/consts/promote_const_let.rs ... ok
---
2019-10-25T17:23:08.8486593Z test [ui] ui/wrapping-int-combinations.rs ... ok
2019-10-25T17:23:08.8486719Z 
2019-10-25T17:23:08.8486789Z failures:
2019-10-25T17:23:08.8521090Z 
2019-10-25T17:23:08.8521514Z ---- [ui] ui/consts/offset_from_ub.rs stdout ----
2019-10-25T17:23:08.8521631Z 
2019-10-25T17:23:08.8521684Z 1 error: any use of this value will cause an error
2019-10-25T17:23:08.8521684Z 1 error: any use of this value will cause an error
2019-10-25T17:23:08.8521907Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-10-25T17:23:08.8522071Z -    |
2019-10-25T17:23:08.8522309Z - LL |           intrinsics::ptr_offset_from(self, origin)
2019-10-25T17:23:08.8522703Z -    |           |
2019-10-25T17:23:08.8522937Z -    |           ptr_offset_from cannot compute offset of pointers into different allocations.
2019-10-25T17:23:08.8522937Z -    |           ptr_offset_from cannot compute offset of pointers into different allocations.
2019-10-25T17:23:08.8523215Z -    |           inside call to `std::ptr::<impl *const Struct>::offset_from` at $DIR/offset_from_ub.rs:20:27
2019-10-25T17:23:08.8523309Z 9    | 
2019-10-25T17:23:08.8523360Z 10   ::: $DIR/offset_from_ub.rs:14:1
2019-10-25T17:23:08.8523636Z 
2019-10-25T17:23:08.8523709Z 21    = note: `#[deny(const_err)]` on by default
2019-10-25T17:23:08.8523870Z 22 
2019-10-25T17:23:08.8524125Z 23 error: any use of this value will cause an error
2019-10-25T17:23:08.8524125Z 23 error: any use of this value will cause an error
2019-10-25T17:23:08.8524399Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-10-25T17:23:08.8524632Z -    |
2019-10-25T17:23:08.8524892Z - LL |           intrinsics::ptr_offset_from(self, origin)
2019-10-25T17:23:08.8525440Z -    |           |
2019-10-25T17:23:08.8525730Z -    |           a memory access tried to interpret some bytes as a pointer
2019-10-25T17:23:08.8525730Z -    |           a memory access tried to interpret some bytes as a pointer
2019-10-25T17:23:08.8526307Z -    |           inside call to `std::ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:26:14
2019-10-25T17:23:08.8526406Z 31    | 
2019-10-25T17:23:08.8526490Z 32   ::: $DIR/offset_from_ub.rs:24:1
2019-10-25T17:23:08.8526620Z 
2019-10-25T17:23:08.8526839Z 38    | |__-
2019-10-25T17:23:08.8526924Z 39 
2019-10-25T17:23:08.8526996Z 40 error: any use of this value will cause an error
2019-10-25T17:23:08.8526996Z 40 error: any use of this value will cause an error
2019-10-25T17:23:08.8527268Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-10-25T17:23:08.8529032Z -    |
2019-10-25T17:23:08.8529353Z - LL |           intrinsics::ptr_offset_from(self, origin)
2019-10-25T17:23:08.8529971Z -    |           |
2019-10-25T17:23:08.8529971Z -    |           |
2019-10-25T17:23:08.8530286Z -    |           exact_div: 1 cannot be divided by 2 without remainder
2019-10-25T17:23:08.8530565Z -    |           inside call to `std::ptr::<impl *const u16>::offset_from` at $DIR/offset_from_ub.rs:34:27
2019-10-25T17:23:08.8530672Z 48    | 
2019-10-25T17:23:08.8530724Z 49   ::: $DIR/offset_from_ub.rs:29:1
2019-10-25T17:23:08.8530832Z 
2019-10-25T17:23:08.8530860Z 
2019-10-25T17:23:08.8530934Z The actual stderr differed from the expected stderr.
2019-10-25T17:23:08.8530934Z The actual stderr differed from the expected stderr.
2019-10-25T17:23:08.8531239Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/offset_from_ub.stderr
2019-10-25T17:23:08.8531481Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T17:23:08.8531908Z To only update this specific test, also pass `--test-args consts/offset_from_ub.rs`
2019-10-25T17:23:08.8532026Z error: 1 errors occurred comparing output.
2019-10-25T17:23:08.8532089Z status: exit code: 1
2019-10-25T17:23:08.8532089Z status: exit code: 1
2019-10-25T17:23:08.8532803Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary" "-A" "unused"
2019-10-25T17:23:08.8533226Z ------------------------------------------
2019-10-25T17:23:08.8533284Z 
2019-10-25T17:23:08.8533650Z ------------------------------------------
2019-10-25T17:23:08.8534107Z stderr:
2019-10-25T17:23:08.8534107Z stderr:
2019-10-25T17:23:08.8534394Z ------------------------------------------
2019-10-25T17:23:08.8534495Z error: any use of this value will cause an error
2019-10-25T17:23:08.8534580Z    | 
2019-10-25T17:23:08.8534670Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:14:1
2019-10-25T17:23:08.8534768Z    |
2019-10-25T17:23:08.8534839Z LL | / pub const DIFFERENT_ALLOC: usize = {
2019-10-25T17:23:08.8534937Z LL | |     //~^ NOTE
2019-10-25T17:23:08.8535018Z LL | |     let uninit = std::mem::MaybeUninit::<Struct>::uninit();
2019-10-25T17:23:08.8535133Z LL | |     let base_ptr: *const Struct = &uninit as *const _ as *const Struct;
2019-10-25T17:23:08.8535297Z LL | |     offset as usize
2019-10-25T17:23:08.8535386Z LL | | };
2019-10-25T17:23:08.8535607Z    | |__-
2019-10-25T17:23:08.8535688Z    |
2019-10-25T17:23:08.8535688Z    |
2019-10-25T17:23:08.8535757Z    = note: `#[deny(const_err)]` on by default
2019-10-25T17:23:08.8535826Z 
2019-10-25T17:23:08.8535896Z error: any use of this value will cause an error
2019-10-25T17:23:08.8535987Z    | 
2019-10-25T17:23:08.8536058Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:24:1
2019-10-25T17:23:08.8536149Z    |
2019-10-25T17:23:08.8536222Z LL | / pub const NOT_PTR: usize = {
2019-10-25T17:23:08.8536312Z LL | |     //~^ NOTE
2019-10-25T17:23:08.8536393Z LL | |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }
2019-10-25T17:23:08.8536861Z    | |__-
2019-10-25T17:23:08.8536921Z 
2019-10-25T17:23:08.8536990Z error: any use of this value will cause an error
2019-10-25T17:23:08.8537081Z    | 
2019-10-25T17:23:08.8537081Z    | 
2019-10-25T17:23:08.8537152Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:29:1
2019-10-25T17:23:08.8537245Z    |
2019-10-25T17:23:08.8537330Z LL | / pub const NOT_MULTIPLE_OF_SIZE: usize = {
2019-10-25T17:23:08.8537404Z LL | |     //~^ NOTE
2019-10-25T17:23:08.8537491Z LL | |     let data = [5u8, 6, 7];
2019-10-25T17:23:08.8537802Z ...  |
2019-10-25T17:23:08.8537850Z LL | |     offset as usize
2019-10-25T17:23:08.8537920Z LL | | };
2019-10-25T17:23:08.8538087Z    | |__-
---
2019-10-25T17:23:08.8538607Z 
2019-10-25T17:23:08.8538643Z 
2019-10-25T17:23:08.8538670Z 
2019-10-25T17:23:08.8538733Z failures:
2019-10-25T17:23:08.8538782Z     [ui] ui/consts/offset_from_ub.rs
2019-10-25T17:23:08.8539073Z test result: FAILED. 9193 passed; 1 failed; 60 ignored; 0 measured; 0 filtered out
2019-10-25T17:23:08.8575850Z 
2019-10-25T17:23:08.8626094Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-25T17:23:08.8626263Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-25T17:23:08.8626263Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-25T17:23:08.8626326Z 
2019-10-25T17:23:08.8626376Z 
2019-10-25T17:23:08.8628924Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-25T17:23:08.8629505Z 
2019-10-25T17:23:08.8629547Z 
2019-10-25T17:23:08.8629901Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-10-25T17:23:08.8629998Z Build completed unsuccessfully in 1:09:18
2019-10-25T17:23:08.8629998Z Build completed unsuccessfully in 1:09:18
2019-10-25T17:23:09.4818575Z == clock drift check ==
2019-10-25T17:23:09.4818689Z   local time: Fri Oct 25 17:23:08 UTC 2019
2019-10-25T17:23:09.4818767Z   network time: Fri, 25 Oct 2019 17:23:09 GMT
2019-10-25T17:23:09.4818823Z == end clock drift check ==
2019-10-25T17:23:09.9842747Z 
2019-10-25T17:23:09.9909921Z ##[error]Bash exited with code '1'.
2019-10-25T17:23:09.9949629Z ##[section]Starting: Upload CPU usage statistics
2019-10-25T17:23:09.9956237Z ==============================================================================
2019-10-25T17:23:09.9956340Z Task         : Bash
2019-10-25T17:23:09.9956438Z Description  : Run a Bash script on macOS, Linux, or Windows
