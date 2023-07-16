plain
2020-04-28T11:54:08.5077307Z ========================== Starting Command Output ===========================
2020-04-28T11:54:08.5080774Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c36dea52-1be8-40aa-b21d-076b8b9225d2.sh
2020-04-28T11:54:08.5081183Z 
2020-04-28T11:54:08.5085518Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-28T11:54:08.5103279Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71542/merge to s
2020-04-28T11:54:08.5106348Z Task         : Get sources
2020-04-28T11:54:08.5106648Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T11:54:08.5106922Z Version      : 1.0.0
2020-04-28T11:54:08.5107111Z Author       : Microsoft
---
2020-04-28T11:54:11.5147875Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-28T11:54:11.5156198Z ##[command]git config gc.auto 0
2020-04-28T11:54:11.5160551Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-28T11:54:11.5165595Z ##[command]git config --get-all http.proxy
2020-04-28T11:54:11.5174864Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71542/merge:refs/remotes/pull/71542/merge
---
2020-04-28T11:57:42.4654793Z  ---> cb2676f08729
2020-04-28T11:57:42.4658220Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-28T11:57:42.4674921Z  ---> Using cache
2020-04-28T11:57:42.4677505Z  ---> df25ce111862
2020-04-28T11:57:42.4678790Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-28T11:57:42.4680236Z  ---> 599b9ac96b27
2020-04-28T11:57:42.4680610Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-28T11:57:42.4681596Z  ---> Using cache
2020-04-28T11:57:42.4682007Z  ---> 091087e35a36
---
2020-04-28T11:57:42.5232923Z Looks like docker image is the same as before, not uploading
2020-04-28T11:57:50.9444985Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T11:57:50.9797926Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T11:57:50.9827187Z == clock drift check ==
2020-04-28T11:57:50.9836872Z   local time: Tue Apr 28 11:57:50 UTC 2020
2020-04-28T11:57:51.2828050Z   network time: Tue, 28 Apr 2020 11:57:51 GMT
2020-04-28T11:57:51.2854748Z Starting sccache server...
2020-04-28T11:57:51.3725710Z configure: processing command line
2020-04-28T11:57:51.3725969Z configure: 
2020-04-28T11:57:51.3726880Z configure: rust.dist-src        := False
---
2020-04-28T12:00:08.6824867Z    Compiling unicode-width v0.1.6
2020-04-28T12:00:08.7698435Z    Compiling getopts v0.2.21
2020-04-28T12:00:18.6176927Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-28T12:00:26.6629833Z     Finished release [optimized] target(s) in 1m 00s
2020-04-28T12:00:26.6636588Z {"reason":"build-finished","success":true}
2020-04-28T12:00:26.6875373Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T12:00:27.2528269Z    Compiling cfg-if v0.1.10
2020-04-28T12:00:27.2540746Z    Compiling libc v0.2.69
2020-04-28T12:00:27.3087879Z    Compiling semver-parser v0.7.0
---
2020-04-28T12:02:56.5098987Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-28T12:02:57.8805522Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-28T12:02:59.4458023Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-28T12:03:00.2111077Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-28T12:03:08.9132715Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-28T12:03:11.1619618Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-28T12:03:15.2394023Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-28T12:03:19.0907149Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-28T12:03:28.9813282Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-28T12:18:33.3862351Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-28T12:18:34.4534298Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-28T12:21:00.6282557Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-28T12:21:01.2445945Z     Finished release [optimized] target(s) in 20m 34s
2020-04-28T12:21:01.2449925Z {"reason":"build-finished","success":true}
2020-04-28T12:21:01.3022347Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-28T12:21:01.3043457Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T12:21:01.5823659Z    Compiling cc v1.0.50
2020-04-28T12:21:01.5836583Z    Compiling core v0.0.0 (/checkout/src/libcore)
2020-04-28T12:21:01.5836583Z    Compiling core v0.0.0 (/checkout/src/libcore)
2020-04-28T12:21:05.1550896Z error: identifier pair considered confusable between `_mm_load1_pd` and `_mm_loadl_pd`
2020-04-28T12:21:05.1555246Z      |
2020-04-28T12:21:05.1555246Z      |
2020-04-28T12:21:05.1555962Z 2571 | pub unsafe fn _mm_loadl_pd(a: __m128d, mem_addr: *const f64) -> __m128d {
2020-04-28T12:21:05.1557326Z ...
2020-04-28T12:21:05.1557326Z ...
2020-04-28T12:21:05.1557960Z 2704 | pub unsafe fn _mm_load1_pd(mem_addr: *const f64) -> __m128d {
2020-04-28T12:21:05.1558831Z      |               ------------ this is where the previous identifier occurred
2020-04-28T12:21:05.1560147Z      = note: `-D confusable-idents` implied by `-D warnings`
2020-04-28T12:21:05.1560437Z 
2020-04-28T12:21:05.1560437Z 
2020-04-28T12:21:05.1577047Z error: identifier pair considered confusable between `_mm_store1_pd` and `_mm_storel_pd`
2020-04-28T12:21:05.1578348Z      |
2020-04-28T12:21:05.1578348Z      |
2020-04-28T12:21:05.1579010Z 2638 | pub unsafe fn _mm_store1_pd(mem_addr: *mut f64, a: __m128d) {
2020-04-28T12:21:05.1579894Z      |               ------------- this is where the previous identifier occurred
2020-04-28T12:21:05.1580469Z ...
2020-04-28T12:21:05.1581114Z 2692 | pub unsafe fn _mm_storel_pd(mem_addr: *mut f64, a: __m128d) {
2020-04-28T12:21:05.1582063Z 
2020-04-28T12:21:05.1606718Z error: identifier pair considered confusable between `e1` and `el`
2020-04-28T12:21:05.1607372Z     --> src/libcore/slice/mod.rs:2166:13
2020-04-28T12:21:05.1607844Z      |
2020-04-28T12:21:05.1607844Z      |
2020-04-28T12:21:05.1608358Z 2166 |         for el in self {
2020-04-28T12:21:05.1608940Z      |             ^^
2020-04-28T12:21:05.1609387Z      | 
2020-04-28T12:21:05.1609965Z     ::: src/libcore/../stdarch/crates/core_arch/src/x86/sse2.rs:991:30
2020-04-28T12:21:05.1610485Z      |
2020-04-28T12:21:05.1611266Z 991  | pub unsafe fn _mm_set_epi64x(e1: i64, e0: i64) -> __m128i {
2020-04-28T12:21:05.1612176Z      |                              -- this is where the previous identifier occurred
2020-04-28T12:21:05.1630454Z error: identifier pair considered confusable between `I` and `l`
2020-04-28T12:21:05.1631119Z     --> src/libcore/slice/mod.rs:5880:13
2020-04-28T12:21:05.1631575Z      |
2020-04-28T12:21:05.1631575Z      |
2020-04-28T12:21:05.1632196Z 5880 |         let l = cmp::min(left.len(), right.len());
2020-04-28T12:21:05.1633294Z      | 
2020-04-28T12:21:05.1633803Z     ::: src/libcore/ptr/mod.rs:1363:45
2020-04-28T12:21:05.1634281Z      |
2020-04-28T12:21:05.1634281Z      |
2020-04-28T12:21:05.1634843Z 1363 | fnptr_impls_args! { A, B, C, D, E, F, G, H, I }
2020-04-28T12:21:05.1635692Z      |                                             - this is where the previous identifier occurred
2020-04-28T12:21:05.1636209Z 
2020-04-28T12:21:05.1690070Z error: identifier pair considered confusable between `test_mm_load1_pd` and `test_mm_loadl_pd`
2020-04-28T12:21:05.1691514Z      |
2020-04-28T12:21:05.1692051Z 4737 |     unsafe fn test_mm_loadl_pd() {
2020-04-28T12:21:05.1692810Z      |               ^^^^^^^^^^^^^^^^
2020-04-28T12:21:05.1693269Z ...
2020-04-28T12:21:05.1693269Z ...
2020-04-28T12:21:05.1693784Z 5063 |     unsafe fn test_mm_load1_pd() {
2020-04-28T12:21:05.1694594Z      |               ---------------- this is where the previous identifier occurred
2020-04-28T12:21:05.1694945Z 
2020-04-28T12:21:05.1706456Z error: identifier pair considered confusable between `test_mm_store1_pd` and `test_mm_storel_pd`
2020-04-28T12:21:05.1707725Z      |
2020-04-28T12:21:05.1708261Z 4805 |     unsafe fn test_mm_store1_pd() {
2020-04-28T12:21:05.1709090Z      |               ----------------- this is where the previous identifier occurred
2020-04-28T12:21:05.1709648Z ...
---
2020-04-28T12:21:13.1995929Z error: could not compile `core`.
2020-04-28T12:21:13.1996219Z 
2020-04-28T12:21:13.1996657Z To learn more, run the command again with --verbose.
2020-04-28T12:21:13.1997247Z warning: build failed, waiting for other jobs to finish...
2020-04-28T12:21:17.6423411Z {"reason":"build-finished","success":false}
2020-04-28T12:21:17.6514091Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-28T12:21:17.6514884Z expected success, got: exit code: 101
2020-04-28T12:21:17.6523654Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-28T12:21:17.6524013Z Build completed unsuccessfully in 0:21:52
2020-04-28T12:21:17.6524013Z Build completed unsuccessfully in 0:21:52
2020-04-28T12:21:17.6576790Z == clock drift check ==
2020-04-28T12:21:17.6599071Z   local time: Tue Apr 28 12:21:17 UTC 2020
2020-04-28T12:21:17.7192610Z   network time: Tue, 28 Apr 2020 12:21:17 GMT
2020-04-28T12:21:19.4438326Z 
2020-04-28T12:21:19.4438326Z 
2020-04-28T12:21:19.4516128Z ##[error]Bash exited with code '1'.
2020-04-28T12:21:19.4528905Z ##[section]Finishing: Run build
2020-04-28T12:21:19.4569140Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71542/merge to s
2020-04-28T12:21:19.4575644Z Task         : Get sources
2020-04-28T12:21:19.4575955Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T12:21:19.4576236Z Version      : 1.0.0
2020-04-28T12:21:19.4576461Z Author       : Microsoft
2020-04-28T12:21:19.4576461Z Author       : Microsoft
2020-04-28T12:21:19.4576792Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-28T12:21:19.4577150Z ==============================================================================
2020-04-28T12:21:19.7776012Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-28T12:21:19.7821839Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71542/merge to s
2020-04-28T12:21:19.7909982Z Cleaning up task key
2020-04-28T12:21:19.7911246Z Start cleaning up orphan processes.
2020-04-28T12:21:19.8087970Z Terminate orphan process: pid (4071) (python)
2020-04-28T12:21:19.8240393Z ##[section]Finishing: Finalize Job
