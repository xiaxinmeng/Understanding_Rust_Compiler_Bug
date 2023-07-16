plain
2019-12-14T10:43:30.6893603Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-14T10:43:30.7064963Z ##[command]git config gc.auto 0
2019-12-14T10:43:30.7143478Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-14T10:43:30.7193848Z ##[command]git config --get-all http.proxy
2019-12-14T10:43:30.7329775Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67163/merge:refs/remotes/pull/67163/merge
---
2019-12-14T11:40:39.1626067Z .................................................................................................... 1600/9375
2019-12-14T11:40:43.3002641Z ....................................................................................F............... 1700/9375
2019-12-14T11:40:54.9387689Z ................................................................i................................... 1800/9375
2019-12-14T11:41:02.5432556Z .................................................................................................... 1900/9375
2019-12-14T11:41:17.0880475Z .................................................iiiii.............................................. 2000/9375
2019-12-14T11:41:27.3586009Z .................................................................................................... 2200/9375
2019-12-14T11:41:29.6084068Z .................................................................................................... 2300/9375
2019-12-14T11:41:32.8859199Z .................................................................................................... 2400/9375
2019-12-14T11:41:54.6578437Z .................................................................................................... 2500/9375
---
2019-12-14T11:44:26.1592547Z .................................................................................................... 4700/9375
2019-12-14T11:44:31.3298823Z .........................................................i...............i.......................... 4800/9375
2019-12-14T11:44:39.2314157Z .................................................................................................... 4900/9375
2019-12-14T11:44:47.4084208Z .................................................................................................... 5000/9375
2019-12-14T11:44:52.5138355Z .i.................................................................................................. 5100/9375
2019-12-14T11:45:02.8343051Z ...................................................................ii.ii...........i................ 5200/9375
2019-12-14T11:45:11.6280456Z ...i................................................................................................ 5400/9375
2019-12-14T11:45:22.0025100Z .................................................................................................... 5500/9375
2019-12-14T11:45:28.3389193Z .................................................i.................................................. 5600/9375
2019-12-14T11:45:35.4637599Z .................................................................................................... 5700/9375
2019-12-14T11:45:35.4637599Z .................................................................................................... 5700/9375
2019-12-14T11:45:45.6882973Z .................................................................................................... 5800/9375
2019-12-14T11:45:55.6153428Z .....................................ii...i..ii...........i......................................... 5900/9375
2019-12-14T11:46:14.3688672Z .................................................................................................... 6100/9375
2019-12-14T11:46:22.3917388Z .................................................................................................... 6200/9375
2019-12-14T11:46:22.3917388Z .................................................................................................... 6200/9375
2019-12-14T11:46:32.0335219Z .............................................................i..ii.................................. 6300/9375
2019-12-14T11:46:59.1242700Z .................................................................................................... 6500/9375
2019-12-14T11:47:01.1281798Z .................................i.................................................................. 6600/9375
2019-12-14T11:47:03.3390496Z .................................................................................................... 6700/9375
2019-12-14T11:47:05.6052471Z .........................i.......................................................................... 6800/9375
---
2019-12-14T11:48:40.8244505Z .................................................................................................... 7400/9375
2019-12-14T11:48:45.8219009Z .................................................................................................... 7500/9375
2019-12-14T11:48:51.2196881Z .................................................................................................... 7600/9375
2019-12-14T11:49:00.3452840Z .................................................................................................... 7700/9375
2019-12-14T11:49:08.6771438Z ...............................................iiii................................................. 7800/9375
2019-12-14T11:49:22.6422832Z .................................................................................................... 8000/9375
2019-12-14T11:49:30.7366775Z .................................................................................................... 8100/9375
2019-12-14T11:49:44.4753706Z .................................................................................................... 8200/9375
2019-12-14T11:49:52.4883781Z .................................................................................................... 8300/9375
---
2019-12-14T11:51:51.1871281Z 
2019-12-14T11:51:51.1871860Z ---- [ui] ui/consts/offset_from_ub.rs stdout ----
2019-12-14T11:51:51.1872085Z diff of stderr:
2019-12-14T11:51:51.1872117Z 
2019-12-14T11:51:51.1872179Z 1 error: any use of this value will cause an error
2019-12-14T11:51:51.1872427Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-14T11:51:51.1872637Z +   --> $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
2019-12-14T11:51:51.1872681Z 3    |
2019-12-14T11:51:51.1872743Z 4 LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-14T11:51:51.1873028Z 
2019-12-14T11:51:51.1873088Z 6    |           |
2019-12-14T11:51:51.1873138Z 7    |           ptr_offset_from cannot compute offset of pointers into different allocations.
2019-12-14T11:51:51.1873138Z 7    |           ptr_offset_from cannot compute offset of pointers into different allocations.
2019-12-14T11:51:51.1873466Z -    |           inside call to `std::ptr::<impl *const Struct>::offset_from` at $DIR/offset_from_ub.rs:19:27
2019-12-14T11:51:51.1873545Z +    |           inside call to `std::ptr::const_ptr::<impl *const Struct>::offset_from` at $DIR/offset_from_ub.rs:19:27
2019-12-14T11:51:51.1873592Z 9    | 
2019-12-14T11:51:51.1873634Z 10   ::: $DIR/offset_from_ub.rs:13:1
2019-12-14T11:51:51.1873718Z 
2019-12-14T11:51:51.1873760Z 21    = note: `#[deny(const_err)]` on by default
2019-12-14T11:51:51.1873819Z 22 
2019-12-14T11:51:51.1873863Z 23 error: any use of this value will cause an error
2019-12-14T11:51:51.1873863Z 23 error: any use of this value will cause an error
2019-12-14T11:51:51.1874067Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-14T11:51:51.1874278Z +   --> $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
2019-12-14T11:51:51.1874385Z 25    |
2019-12-14T11:51:51.1874430Z 26 LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-14T11:51:51.1874681Z 
2019-12-14T11:51:51.1874719Z 28    |           |
2019-12-14T11:51:51.1874927Z 29    |           a memory access tried to interpret some bytes as a pointer
2019-12-14T11:51:51.1874927Z 29    |           a memory access tried to interpret some bytes as a pointer
2019-12-14T11:51:51.1875205Z -    |           inside call to `std::ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:25:14
2019-12-14T11:51:51.1875261Z +    |           inside call to `std::ptr::const_ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:25:14
2019-12-14T11:51:51.1875304Z 31    | 
2019-12-14T11:51:51.1875342Z 32   ::: $DIR/offset_from_ub.rs:23:1
2019-12-14T11:51:51.1875419Z 
2019-12-14T11:51:51.1875578Z 38    | |__-
2019-12-14T11:51:51.1875630Z 39 
2019-12-14T11:51:51.1875671Z 40 error: any use of this value will cause an error
2019-12-14T11:51:51.1875671Z 40 error: any use of this value will cause an error
2019-12-14T11:51:51.1875862Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-14T11:51:51.1877150Z +   --> $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
2019-12-14T11:51:51.1877235Z 42    |
2019-12-14T11:51:51.1877280Z 43 LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-14T11:51:51.1877381Z 
2019-12-14T11:51:51.1877421Z 45    |           |
2019-12-14T11:51:51.1877421Z 45    |           |
2019-12-14T11:51:51.1877471Z 46    |           exact_div: 1 cannot be divided by 2 without remainder
2019-12-14T11:51:51.1877839Z -    |           inside call to `std::ptr::<impl *const u16>::offset_from` at $DIR/offset_from_ub.rs:33:14
2019-12-14T11:51:51.1877903Z +    |           inside call to `std::ptr::const_ptr::<impl *const u16>::offset_from` at $DIR/offset_from_ub.rs:33:14
2019-12-14T11:51:51.1877951Z 48    | 
2019-12-14T11:51:51.1878011Z 49   ::: $DIR/offset_from_ub.rs:28:1
2019-12-14T11:51:51.1878082Z 
2019-12-14T11:51:51.1878394Z 58    | |__-
2019-12-14T11:51:51.1878479Z 59 
2019-12-14T11:51:51.1878526Z 60 error: any use of this value will cause an error
2019-12-14T11:51:51.1878526Z 60 error: any use of this value will cause an error
2019-12-14T11:51:51.1878779Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-14T11:51:51.1878997Z +   --> $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
2019-12-14T11:51:51.1879058Z 62    |
2019-12-14T11:51:51.1879105Z 63 LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-14T11:51:51.1879202Z 
2019-12-14T11:51:51.1879243Z 65    |           |
2019-12-14T11:51:51.1879290Z 66    |           invalid use of NULL pointer
2019-12-14T11:51:51.1879290Z 66    |           invalid use of NULL pointer
2019-12-14T11:51:51.1879589Z -    |           inside call to `std::ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:39:14
2019-12-14T11:51:51.1879652Z +    |           inside call to `std::ptr::const_ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:39:14
2019-12-14T11:51:51.1879703Z 68    | 
2019-12-14T11:51:51.1879764Z 69   ::: $DIR/offset_from_ub.rs:36:1
2019-12-14T11:51:51.1879932Z 
2019-12-14T11:51:51.1880144Z 76    | |__-
2019-12-14T11:51:51.1880205Z 77 
2019-12-14T11:51:51.1880252Z 78 error: any use of this value will cause an error
2019-12-14T11:51:51.1880252Z 78 error: any use of this value will cause an error
2019-12-14T11:51:51.1880463Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-14T11:51:51.1881032Z +   --> $SRC_DIR/libcore/ptr/const_ptr.rs:LL:COL
2019-12-14T11:51:51.1881247Z 80    |
2019-12-14T11:51:51.1881286Z 81 LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-14T11:51:51.1881375Z 
2019-12-14T11:51:51.1881413Z 83    |           |
2019-12-14T11:51:51.1881457Z 84    |           a memory access tried to interpret some bytes as a pointer
2019-12-14T11:51:51.1881457Z 84    |           a memory access tried to interpret some bytes as a pointer
2019-12-14T11:51:51.1881731Z -    |           inside call to `std::ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:46:14
2019-12-14T11:51:51.1881796Z +    |           inside call to `std::ptr::const_ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:46:14
2019-12-14T11:51:51.1882007Z 86    | 
2019-12-14T11:51:51.1882061Z 87   ::: $DIR/offset_from_ub.rs:42:1
2019-12-14T11:51:51.1882120Z 
2019-12-14T11:51:51.1882143Z 
2019-12-14T11:51:51.1882183Z The actual stderr differed from the expected stderr.
2019-12-14T11:51:51.1882183Z The actual stderr differed from the expected stderr.
2019-12-14T11:51:51.1882466Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/offset_from_ub.stderr
2019-12-14T11:51:51.1882850Z To update references, rerun the tests and pass the `--bless` flag
2019-12-14T11:51:51.1883081Z To only update this specific test, also pass `--test-args consts/offset_from_ub.rs`
2019-12-14T11:51:51.1883168Z error: 1 errors occurred comparing output.
2019-12-14T11:51:51.1883208Z status: exit code: 1
2019-12-14T11:51:51.1883208Z status: exit code: 1
2019-12-14T11:51:51.1883911Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary" "-A" "unused"
2019-12-14T11:51:51.1884205Z ------------------------------------------
2019-12-14T11:51:51.1884236Z 
2019-12-14T11:51:51.1884428Z ------------------------------------------
2019-12-14T11:51:51.1884483Z stderr:
2019-12-14T11:51:51.1884483Z stderr:
2019-12-14T11:51:51.1884670Z ------------------------------------------
2019-12-14T11:51:51.1884714Z error: any use of this value will cause an error
2019-12-14T11:51:51.1884915Z   --> /checkout/src/libcore/ptr/const_ptr.rs:294:9
2019-12-14T11:51:51.1884972Z    |
2019-12-14T11:51:51.1885174Z LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-14T11:51:51.1885602Z    |           |
2019-12-14T11:51:51.1885653Z    |           ptr_offset_from cannot compute offset of pointers into different allocations.
2019-12-14T11:51:51.1885718Z    |           inside call to `std::ptr::const_ptr::<impl *const Struct>::offset_from` at /checkout/src/test/ui/consts/offset_from_ub.rs:19:27
2019-12-14T11:51:51.1885789Z    | 
2019-12-14T11:51:51.1885789Z    | 
2019-12-14T11:51:51.1885837Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:13:1
2019-12-14T11:51:51.1885900Z    |
2019-12-14T11:51:51.1885945Z LL | / pub const DIFFERENT_ALLOC: usize = {
2019-12-14T11:51:51.1885991Z LL | |     //~^ NOTE
2019-12-14T11:51:51.1886039Z LL | |     let uninit = std::mem::MaybeUninit::<Struct>::uninit();
2019-12-14T11:51:51.1886111Z LL | |     let base_ptr: *const Struct = &uninit as *const _ as *const Struct;
2019-12-14T11:51:51.1886199Z LL | |     offset as usize
2019-12-14T11:51:51.1886262Z LL | | };
2019-12-14T11:51:51.1886512Z    | |__-
2019-12-14T11:51:51.1886636Z    |
2019-12-14T11:51:51.1886636Z    |
2019-12-14T11:51:51.1886699Z    = note: `#[deny(const_err)]` on by default
2019-12-14T11:51:51.1886731Z 
2019-12-14T11:51:51.1886775Z error: any use of this value will cause an error
2019-12-14T11:51:51.1887033Z   --> /checkout/src/libcore/ptr/const_ptr.rs:294:9
2019-12-14T11:51:51.1887098Z    |
2019-12-14T11:51:51.1887145Z LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-14T11:51:51.1887257Z    |           |
2019-12-14T11:51:51.1887306Z    |           a memory access tried to interpret some bytes as a pointer
2019-12-14T11:51:51.1887367Z    |           inside call to `std::ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/src/test/ui/consts/offset_from_ub.rs:25:14
2019-12-14T11:51:51.1887435Z    | 
2019-12-14T11:51:51.1887435Z    | 
2019-12-14T11:51:51.1887482Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:23:1
2019-12-14T11:51:51.1887525Z    |
2019-12-14T11:51:51.1887576Z LL | / pub const NOT_PTR: usize = {
2019-12-14T11:51:51.1887646Z LL | |     //~^ NOTE
2019-12-14T11:51:51.1887694Z LL | |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }
2019-12-14T11:51:51.1887937Z    | |__-
2019-12-14T11:51:51.1887966Z 
2019-12-14T11:51:51.1888010Z error: any use of this value will cause an error
2019-12-14T11:51:51.1888232Z   --> /checkout/src/libcore/ptr/const_ptr.rs:294:9
2019-12-14T11:51:51.1888232Z   --> /checkout/src/libcore/ptr/const_ptr.rs:294:9
2019-12-14T11:51:51.1888293Z    |
2019-12-14T11:51:51.1888338Z LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-14T11:51:51.1888450Z    |           |
2019-12-14T11:51:51.1888450Z    |           |
2019-12-14T11:51:51.1888497Z    |           exact_div: 1 cannot be divided by 2 without remainder
2019-12-14T11:51:51.1888557Z    |           inside call to `std::ptr::const_ptr::<impl *const u16>::offset_from` at /checkout/src/test/ui/consts/offset_from_ub.rs:33:14
2019-12-14T11:51:51.1888680Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:28:1
2019-12-14T11:51:51.1888731Z    |
2019-12-14T11:51:51.1888731Z    |
2019-12-14T11:51:51.1889226Z LL | / pub const NOT_MULTIPLE_OF_SIZE: isize = {
2019-12-14T11:51:51.1889269Z LL | |     //~^ NOTE
2019-12-14T11:51:51.1889473Z LL | |     let data = [5u8, 6, 7];
2019-12-14T11:51:51.1889531Z LL | |     let base_ptr = data.as_ptr();
2019-12-14T11:51:51.1889574Z LL | |     let field_ptr = &data[1] as *const u8 as *const u16;
2019-12-14T11:51:51.1889618Z LL | |     unsafe { field_ptr.offset_from(base_ptr as *const u16) }
2019-12-14T11:51:51.1889843Z    | |__-
2019-12-14T11:51:51.1889870Z 
2019-12-14T11:51:51.1889907Z error: any use of this value will cause an error
2019-12-14T11:51:51.1890113Z   --> /checkout/src/libcore/ptr/const_ptr.rs:294:9
2019-12-14T11:51:51.1890113Z   --> /checkout/src/libcore/ptr/const_ptr.rs:294:9
2019-12-14T11:51:51.1890152Z    |
2019-12-14T11:51:51.1890190Z LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-14T11:51:51.1890366Z    |           |
2019-12-14T11:51:51.1890412Z    |           invalid use of NULL pointer
2019-12-14T11:51:51.1890462Z    |           inside call to `std::ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/src/test/ui/consts/offset_from_ub.rs:39:14
2019-12-14T11:51:51.1890520Z    | 
2019-12-14T11:51:51.1890520Z    | 
2019-12-14T11:51:51.1890559Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:36:1
2019-12-14T11:51:51.1890596Z    |
2019-12-14T11:51:51.1890646Z LL | / pub const OFFSET_FROM_NULL: isize = {
2019-12-14T11:51:51.1890685Z LL | |     //~^ NOTE
2019-12-14T11:51:51.1890746Z LL | |     let ptr = 0 as *const u8;
2019-12-14T11:51:51.1890786Z LL | |     unsafe { ptr.offset_from(ptr) }
2019-12-14T11:51:51.1891029Z    | |__-
2019-12-14T11:51:51.1891055Z 
2019-12-14T11:51:51.1891094Z error: any use of this value will cause an error
2019-12-14T11:51:51.1891290Z   --> /checkout/src/libcore/ptr/const_ptr.rs:294:9
2019-12-14T11:51:51.1891290Z   --> /checkout/src/libcore/ptr/const_ptr.rs:294:9
2019-12-14T11:51:51.1891346Z    |
2019-12-14T11:51:51.1891392Z LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-14T11:51:51.1891563Z    |           |
2019-12-14T11:51:51.1891604Z    |           a memory access tried to interpret some bytes as a pointer
2019-12-14T11:51:51.1891657Z    |           inside call to `std::ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/src/test/ui/consts/offset_from_ub.rs:46:14
2019-12-14T11:51:51.1891718Z    | 
2019-12-14T11:51:51.1891718Z    | 
2019-12-14T11:51:51.1891758Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:42:1
2019-12-14T11:51:51.1891795Z    |
2019-12-14T11:51:51.1891838Z LL | / pub const DIFFERENT_INT: isize = { // offset_from with two different integers: like DIFFERENT_ALLOC
2019-12-14T11:51:51.1891900Z LL | |     //~^ NOTE
2019-12-14T11:51:51.1891938Z LL | |     let ptr1 = 8 as *const u8;
2019-12-14T11:51:51.1891977Z LL | |     let ptr2 = 16 as *const u8;
2019-12-14T11:51:51.1892033Z LL | |     unsafe { ptr2.offset_from(ptr1) }
2019-12-14T11:51:51.1892269Z    | |__-
2019-12-14T11:51:51.1939731Z 
2019-12-14T11:51:51.1939862Z error: aborting due to 5 previous errors
2019-12-14T11:51:51.1939892Z 
---
2019-12-14T11:51:51.1941321Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-14T11:51:51.1941375Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-14T11:51:51.1967308Z 
2019-12-14T11:51:51.1967420Z 
2019-12-14T11:51:51.1969857Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-14T11:51:51.1970145Z 
2019-12-14T11:51:51.1970176Z 
2019-12-14T11:51:51.1975844Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-14T11:51:51.1975915Z Build completed unsuccessfully in 1:02:36
2019-12-14T11:51:51.1975915Z Build completed unsuccessfully in 1:02:36
2019-12-14T11:51:51.2038448Z == clock drift check ==
2019-12-14T11:51:51.2058592Z   local time: Sat Dec 14 11:51:51 UTC 2019
2019-12-14T11:51:51.5403681Z   network time: Sat, 14 Dec 2019 11:51:51 GMT
2019-12-14T11:51:51.5404112Z == end clock drift check ==
2019-12-14T11:51:52.3271733Z 
2019-12-14T11:51:52.3383114Z ##[error]Bash exited with code '1'.
2019-12-14T11:51:52.3427538Z ##[section]Starting: Checkout
2019-12-14T11:51:52.3429540Z ==============================================================================
2019-12-14T11:51:52.3429701Z Task         : Get sources
2019-12-14T11:51:52.3429753Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
