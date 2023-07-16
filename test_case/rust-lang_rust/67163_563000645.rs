plain
2019-12-08T20:52:43.4830419Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-08T20:52:43.5040454Z ##[command]git config gc.auto 0
2019-12-08T20:52:44.2849581Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-08T20:52:44.2859875Z ##[command]git config --get-all http.proxy
2019-12-08T20:52:44.2866731Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67163/merge:refs/remotes/pull/67163/merge
---
2019-12-08T21:54:09.1049560Z .................................................................................................... 1600/9337
2019-12-08T21:54:13.7853862Z ...................................................................F................................ 1700/9337
2019-12-08T21:54:25.9341807Z ...............................................i.................................................... 1800/9337
2019-12-08T21:54:34.1210427Z .................................................................................................... 1900/9337
2019-12-08T21:54:48.1553508Z ................................iiiii............................................................... 2000/9337
2019-12-08T21:54:58.1952226Z .................................................................................................... 2200/9337
2019-12-08T21:55:00.7407554Z .................................................................................................... 2300/9337
2019-12-08T21:55:05.1018637Z .................................................................................................... 2400/9337
2019-12-08T21:55:26.9711939Z .................................................................................................... 2500/9337
---
2019-12-08T21:58:06.2301119Z ...................................i...............i................................................ 4800/9337
2019-12-08T21:58:15.9874297Z .................................................................................................... 4900/9337
2019-12-08T21:58:22.8196362Z ...............................................................................i.................... 5000/9337
2019-12-08T21:58:29.0789663Z .................................................................................................... 5100/9337
2019-12-08T21:58:38.6304950Z ............................................ii.ii...........i....................................... 5200/9337
2019-12-08T21:58:48.2374390Z .................................................................................................... 5400/9337
2019-12-08T21:58:58.0403756Z .................................................................................................... 5500/9337
2019-12-08T21:59:05.1624480Z ..........................i......................................................................... 5600/9337
2019-12-08T21:59:11.4462159Z .................................................................................................... 5700/9337
2019-12-08T21:59:11.4462159Z .................................................................................................... 5700/9337
2019-12-08T21:59:23.3890425Z .................................................................................................... 5800/9337
2019-12-08T21:59:34.6298151Z .............ii...i..ii...........i................................................................. 5900/9337
2019-12-08T21:59:52.5768396Z .................................................................................................... 6100/9337
2019-12-08T22:00:00.3648276Z .................................................................................................... 6200/9337
2019-12-08T22:00:00.3648276Z .................................................................................................... 6200/9337
2019-12-08T22:00:19.0551927Z ....................................i..ii........................................................... 6300/9337
2019-12-08T22:00:39.9483044Z .................................................................................................... 6500/9337
2019-12-08T22:00:42.0850838Z ........i........................................................................................... 6600/9337
2019-12-08T22:00:44.3786112Z ...................................................................................................i 6700/9337
2019-12-08T22:00:46.9695992Z .................................................................................................... 6800/9337
---
2019-12-08T22:02:26.7609321Z .................................................................................................... 7400/9337
2019-12-08T22:02:31.9252676Z .................................................................................................... 7500/9337
2019-12-08T22:02:39.1790638Z .................................................................................................... 7600/9337
2019-12-08T22:02:50.1228126Z .................................................................................................... 7700/9337
2019-12-08T22:02:56.4300122Z ...............iiii................................................................................. 7800/9337
2019-12-08T22:03:10.5233410Z .................................................................................................... 8000/9337
2019-12-08T22:03:22.1276176Z .................................................................................................... 8100/9337
2019-12-08T22:03:34.8059542Z .................................................................................................... 8200/9337
2019-12-08T22:03:41.6399272Z .................................................................................................... 8300/9337
---
2019-12-08T22:05:36.9681973Z 
2019-12-08T22:05:36.9683101Z ---- [ui] ui/consts/offset_from_ub.rs stdout ----
2019-12-08T22:05:36.9683314Z diff of stderr:
2019-12-08T22:05:36.9683948Z 
2019-12-08T22:05:36.9684093Z 1 error: any use of this value will cause an error
2019-12-08T22:05:36.9684645Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-08T22:05:36.9684938Z +   --> $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
2019-12-08T22:05:36.9685124Z 3    |
2019-12-08T22:05:36.9685171Z 4 LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-08T22:05:36.9685255Z 
2019-12-08T22:05:36.9685313Z 6    |           |
2019-12-08T22:05:36.9685363Z 7    |           ptr_offset_from cannot compute offset of pointers into different allocations.
2019-12-08T22:05:36.9685363Z 7    |           ptr_offset_from cannot compute offset of pointers into different allocations.
2019-12-08T22:05:36.9685706Z -    |           inside call to `std::ptr::<impl *const Struct>::offset_from` at $DIR/offset_from_ub.rs:19:27
2019-12-08T22:05:36.9685791Z +    |           inside call to `std::ptr::const_ptr::<impl *const Struct>::offset_from` at $DIR/offset_from_ub.rs:19:27
2019-12-08T22:05:36.9685841Z 9    | 
2019-12-08T22:05:36.9685886Z 10   ::: $DIR/offset_from_ub.rs:13:1
2019-12-08T22:05:36.9685981Z 
2019-12-08T22:05:36.9686240Z 21    = note: `#[deny(const_err)]` on by default
2019-12-08T22:05:36.9686315Z 22 
2019-12-08T22:05:36.9686360Z 23 error: any use of this value will cause an error
2019-12-08T22:05:36.9686360Z 23 error: any use of this value will cause an error
2019-12-08T22:05:36.9686655Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-08T22:05:36.9686896Z +   --> $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
2019-12-08T22:05:36.9686959Z 25    |
2019-12-08T22:05:36.9687003Z 26 LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-08T22:05:36.9687098Z 
2019-12-08T22:05:36.9687138Z 28    |           |
2019-12-08T22:05:36.9687186Z 29    |           a memory access tried to interpret some bytes as a pointer
2019-12-08T22:05:36.9687186Z 29    |           a memory access tried to interpret some bytes as a pointer
2019-12-08T22:05:36.9687503Z -    |           inside call to `std::ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:25:14
2019-12-08T22:05:36.9687567Z +    |           inside call to `std::ptr::const_ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:25:14
2019-12-08T22:05:36.9687740Z 31    | 
2019-12-08T22:05:36.9687806Z 32   ::: $DIR/offset_from_ub.rs:23:1
2019-12-08T22:05:36.9687875Z 
2019-12-08T22:05:36.9688119Z 38    | |__-
2019-12-08T22:05:36.9688180Z 39 
2019-12-08T22:05:36.9688225Z 40 error: any use of this value will cause an error
2019-12-08T22:05:36.9688225Z 40 error: any use of this value will cause an error
2019-12-08T22:05:36.9688462Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-08T22:05:36.9688714Z +   --> $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
2019-12-08T22:05:36.9688760Z 42    |
2019-12-08T22:05:36.9688803Z 43 LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-08T22:05:36.9688896Z 
2019-12-08T22:05:36.9688936Z 45    |           |
2019-12-08T22:05:36.9688936Z 45    |           |
2019-12-08T22:05:36.9688983Z 46    |           exact_div: 1 cannot be divided by 2 without remainder
2019-12-08T22:05:36.9689297Z -    |           inside call to `std::ptr::<impl *const u16>::offset_from` at $DIR/offset_from_ub.rs:33:14
2019-12-08T22:05:36.9689379Z +    |           inside call to `std::ptr::const_ptr::<impl *const u16>::offset_from` at $DIR/offset_from_ub.rs:33:14
2019-12-08T22:05:36.9689426Z 48    | 
2019-12-08T22:05:36.9689483Z 49   ::: $DIR/offset_from_ub.rs:28:1
2019-12-08T22:05:36.9689550Z 
2019-12-08T22:05:36.9689751Z 58    | |__-
2019-12-08T22:05:36.9689811Z 59 
2019-12-08T22:05:36.9689855Z 60 error: any use of this value will cause an error
2019-12-08T22:05:36.9689855Z 60 error: any use of this value will cause an error
2019-12-08T22:05:36.9690086Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-08T22:05:36.9690334Z +   --> $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
2019-12-08T22:05:36.9690380Z 62    |
2019-12-08T22:05:36.9690424Z 63 LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-08T22:05:36.9690516Z 
2019-12-08T22:05:36.9690556Z 65    |           |
2019-12-08T22:05:36.9690599Z 66    |           invalid use of NULL pointer
2019-12-08T22:05:36.9690599Z 66    |           invalid use of NULL pointer
2019-12-08T22:05:36.9690926Z -    |           inside call to `std::ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:39:14
2019-12-08T22:05:36.9690991Z +    |           inside call to `std::ptr::const_ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:39:14
2019-12-08T22:05:36.9691037Z 68    | 
2019-12-08T22:05:36.9691094Z 69   ::: $DIR/offset_from_ub.rs:36:1
2019-12-08T22:05:36.9691160Z 
2019-12-08T22:05:36.9691360Z 76    | |__-
2019-12-08T22:05:36.9691419Z 77 
2019-12-08T22:05:36.9691462Z 78 error: any use of this value will cause an error
2019-12-08T22:05:36.9691462Z 78 error: any use of this value will cause an error
2019-12-08T22:05:36.9691692Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-08T22:05:36.9691939Z +   --> $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
2019-12-08T22:05:36.9691984Z 80    |
2019-12-08T22:05:36.9692027Z 81 LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-08T22:05:36.9692119Z 
2019-12-08T22:05:36.9692158Z 83    |           |
2019-12-08T22:05:36.9692299Z 84    |           a memory access tried to interpret some bytes as a pointer
2019-12-08T22:05:36.9692299Z 84    |           a memory access tried to interpret some bytes as a pointer
2019-12-08T22:05:36.9692663Z -    |           inside call to `std::ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:46:14
2019-12-08T22:05:36.9692727Z +    |           inside call to `std::ptr::const_ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:46:14
2019-12-08T22:05:36.9692774Z 86    | 
2019-12-08T22:05:36.9692832Z 87   ::: $DIR/offset_from_ub.rs:42:1
2019-12-08T22:05:36.9692899Z 
2019-12-08T22:05:36.9692924Z 
2019-12-08T22:05:36.9692983Z The actual stderr differed from the expected stderr.
2019-12-08T22:05:36.9692983Z The actual stderr differed from the expected stderr.
2019-12-08T22:05:36.9693301Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/offset_from_ub.stderr
2019-12-08T22:05:36.9693896Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T22:05:36.9694207Z To only update this specific test, also pass `--test-args consts/offset_from_ub.rs`
2019-12-08T22:05:36.9694423Z error: 1 errors occurred comparing output.
2019-12-08T22:05:36.9694467Z status: exit code: 1
2019-12-08T22:05:36.9694467Z status: exit code: 1
2019-12-08T22:05:36.9695272Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary" "-A" "unused"
2019-12-08T22:05:36.9695612Z ------------------------------------------
2019-12-08T22:05:36.9695646Z 
2019-12-08T22:05:36.9696389Z ------------------------------------------
2019-12-08T22:05:36.9696470Z stderr:
2019-12-08T22:05:36.9696470Z stderr:
2019-12-08T22:05:36.9696770Z ------------------------------------------
2019-12-08T22:05:36.9696829Z error: any use of this value will cause an error
2019-12-08T22:05:36.9697089Z   --> /checkout/src/libcore/ptr/const_ptr.rs:292:9
2019-12-08T22:05:36.9697137Z    |
2019-12-08T22:05:36.9697181Z LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-08T22:05:36.9697298Z    |           |
2019-12-08T22:05:36.9697349Z    |           ptr_offset_from cannot compute offset of pointers into different allocations.
2019-12-08T22:05:36.9697427Z    |           inside call to `std::ptr::const_ptr::<impl *const Struct>::offset_from` at /checkout/src/test/ui/consts/offset_from_ub.rs:19:27
2019-12-08T22:05:36.9697479Z    | 
2019-12-08T22:05:36.9697479Z    | 
2019-12-08T22:05:36.9697526Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:13:1
2019-12-08T22:05:36.9697584Z    |
2019-12-08T22:05:36.9697628Z LL | / pub const DIFFERENT_ALLOC: usize = {
2019-12-08T22:05:36.9697673Z LL | |     //~^ NOTE
2019-12-08T22:05:36.9697734Z LL | |     let uninit = std::mem::MaybeUninit::<Struct>::uninit();
2019-12-08T22:05:36.9697801Z LL | |     let base_ptr: *const Struct = &uninit as *const _ as *const Struct;
2019-12-08T22:05:36.9697888Z LL | |     offset as usize
2019-12-08T22:05:36.9697945Z LL | | };
2019-12-08T22:05:36.9698157Z    | |__-
2019-12-08T22:05:36.9698200Z    |
2019-12-08T22:05:36.9698200Z    |
2019-12-08T22:05:36.9698259Z    = note: `#[deny(const_err)]` on by default
2019-12-08T22:05:36.9698291Z 
2019-12-08T22:05:36.9698334Z error: any use of this value will cause an error
2019-12-08T22:05:36.9698576Z   --> /checkout/src/libcore/ptr/const_ptr.rs:292:9
2019-12-08T22:05:36.9698638Z    |
2019-12-08T22:05:36.9698682Z LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-08T22:05:36.9698791Z    |           |
2019-12-08T22:05:36.9698838Z    |           a memory access tried to interpret some bytes as a pointer
2019-12-08T22:05:36.9699023Z    |           inside call to `std::ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/src/test/ui/consts/offset_from_ub.rs:25:14
2019-12-08T22:05:36.9699095Z    | 
2019-12-08T22:05:36.9699095Z    | 
2019-12-08T22:05:36.9699140Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:23:1
2019-12-08T22:05:36.9699181Z    |
2019-12-08T22:05:36.9699239Z LL | / pub const NOT_PTR: usize = {
2019-12-08T22:05:36.9699284Z LL | |     //~^ NOTE
2019-12-08T22:05:36.9699330Z LL | |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }
2019-12-08T22:05:36.9699626Z    | |__-
2019-12-08T22:05:36.9699657Z 
2019-12-08T22:05:36.9699701Z error: any use of this value will cause an error
2019-12-08T22:05:36.9699954Z   --> /checkout/src/libcore/ptr/const_ptr.rs:292:9
2019-12-08T22:05:36.9699954Z   --> /checkout/src/libcore/ptr/const_ptr.rs:292:9
2019-12-08T22:05:36.9700000Z    |
2019-12-08T22:05:36.9700044Z LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-08T22:05:36.9700237Z    |           |
2019-12-08T22:05:36.9700237Z    |           |
2019-12-08T22:05:36.9700291Z    |           exact_div: 1 cannot be divided by 2 without remainder
2019-12-08T22:05:36.9700351Z    |           inside call to `std::ptr::const_ptr::<impl *const u16>::offset_from` at /checkout/src/test/ui/consts/offset_from_ub.rs:33:14
2019-12-08T22:05:36.9700460Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:28:1
2019-12-08T22:05:36.9700501Z    |
2019-12-08T22:05:36.9700501Z    |
2019-12-08T22:05:36.9700560Z LL | / pub const NOT_MULTIPLE_OF_SIZE: isize = {
2019-12-08T22:05:36.9700604Z LL | |     //~^ NOTE
2019-12-08T22:05:36.9700647Z LL | |     let data = [5u8, 6, 7];
2019-12-08T22:05:36.9700707Z LL | |     let base_ptr = data.as_ptr();
2019-12-08T22:05:36.9700755Z LL | |     let field_ptr = &data[1] as *const u8 as *const u16;
2019-12-08T22:05:36.9700805Z LL | |     unsafe { field_ptr.offset_from(base_ptr as *const u16) }
2019-12-08T22:05:36.9701097Z    | |__-
2019-12-08T22:05:36.9701128Z 
2019-12-08T22:05:36.9701181Z error: any use of this value will cause an error
2019-12-08T22:05:36.9701444Z   --> /checkout/src/libcore/ptr/const_ptr.rs:292:9
2019-12-08T22:05:36.9701444Z   --> /checkout/src/libcore/ptr/const_ptr.rs:292:9
2019-12-08T22:05:36.9701492Z    |
2019-12-08T22:05:36.9701536Z LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-08T22:05:36.9701641Z    |           |
2019-12-08T22:05:36.9701685Z    |           invalid use of NULL pointer
2019-12-08T22:05:36.9701740Z    |           inside call to `std::ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/src/test/ui/consts/offset_from_ub.rs:39:14
2019-12-08T22:05:36.9701804Z    | 
2019-12-08T22:05:36.9701804Z    | 
2019-12-08T22:05:36.9701848Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:36:1
2019-12-08T22:05:36.9701890Z    |
2019-12-08T22:05:36.9701948Z LL | / pub const OFFSET_FROM_NULL: isize = {
2019-12-08T22:05:36.9701992Z LL | |     //~^ NOTE
2019-12-08T22:05:36.9702035Z LL | |     let ptr = 0 as *const u8;
2019-12-08T22:05:36.9702095Z LL | |     unsafe { ptr.offset_from(ptr) }
2019-12-08T22:05:36.9702356Z    | |__-
2019-12-08T22:05:36.9702386Z 
2019-12-08T22:05:36.9702446Z error: any use of this value will cause an error
2019-12-08T22:05:36.9702687Z   --> /checkout/src/libcore/ptr/const_ptr.rs:292:9
2019-12-08T22:05:36.9702687Z   --> /checkout/src/libcore/ptr/const_ptr.rs:292:9
2019-12-08T22:05:36.9702734Z    |
2019-12-08T22:05:36.9702793Z LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-08T22:05:36.9702882Z    |           |
2019-12-08T22:05:36.9702944Z    |           a memory access tried to interpret some bytes as a pointer
2019-12-08T22:05:36.9703003Z    |           inside call to `std::ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/src/test/ui/consts/offset_from_ub.rs:46:14
2019-12-08T22:05:36.9703051Z    | 
2019-12-08T22:05:36.9703051Z    | 
2019-12-08T22:05:36.9703109Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:42:1
2019-12-08T22:05:36.9703151Z    |
2019-12-08T22:05:36.9703280Z LL | / pub const DIFFERENT_INT: isize = { // offset_from with two different integers: like DIFFERENT_ALLOC
2019-12-08T22:05:36.9703348Z LL | |     //~^ NOTE
2019-12-08T22:05:36.9703693Z LL | |     let ptr1 = 8 as *const u8;
2019-12-08T22:05:36.9703737Z LL | |     let ptr2 = 16 as *const u8;
2019-12-08T22:05:36.9703782Z LL | |     unsafe { ptr2.offset_from(ptr1) }
2019-12-08T22:05:36.9704128Z    | |__-
2019-12-08T22:05:36.9704159Z 
2019-12-08T22:05:36.9704202Z error: aborting due to 5 previous errors
2019-12-08T22:05:36.9704245Z 
---
2019-12-08T22:05:36.9705475Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-08T22:05:36.9705552Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-08T22:05:36.9705584Z 
2019-12-08T22:05:36.9705609Z 
2019-12-08T22:05:36.9707140Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-08T22:05:36.9707368Z 
2019-12-08T22:05:36.9707397Z 
2019-12-08T22:05:36.9707442Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-08T22:05:36.9707506Z Build completed unsuccessfully in 1:06:39
2019-12-08T22:05:36.9707506Z Build completed unsuccessfully in 1:06:39
2019-12-08T22:05:36.9740905Z == clock drift check ==
2019-12-08T22:05:36.9760784Z   local time: Sun Dec  8 22:05:36 UTC 2019
2019-12-08T22:05:37.1328556Z   network time: Sun, 08 Dec 2019 22:05:37 GMT
2019-12-08T22:05:37.1329144Z == end clock drift check ==
2019-12-08T22:05:37.8598181Z 
2019-12-08T22:05:37.8776426Z ##[error]Bash exited with code '1'.
2019-12-08T22:05:37.8821149Z ##[section]Starting: Checkout
2019-12-08T22:05:37.8822939Z ==============================================================================
2019-12-08T22:05:37.8822993Z Task         : Get sources
2019-12-08T22:05:37.8823040Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
