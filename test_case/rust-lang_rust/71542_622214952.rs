plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/45c097bc-d757-437e-9729-fb5c714efecf.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71542/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71542/merge:refs/remotes/pull/71542/merge
---
 ---> cb2676f08729
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> df25ce111862
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 599b9ac96b27
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 091087e35a36
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.50
   Compiling core v0.0.0 (/checkout/src/libcore)
error: identifier pair considered confusable between `_mm_load1_pd` and `_mm_loadl_pd`
     |
     |
2571 | pub unsafe fn _mm_loadl_pd(a: __m128d, mem_addr: *const f64) -> __m128d {
...
...
2704 | pub unsafe fn _mm_load1_pd(mem_addr: *const f64) -> __m128d {
     |               ------------ this is where the previous identifier occurred
     = note: `-D confusable-idents` implied by `-D warnings`


error: identifier pair considered confusable between `_mm_store1_pd` and `_mm_storel_pd`
     |
     |
2638 | pub unsafe fn _mm_store1_pd(mem_addr: *mut f64, a: __m128d) {
     |               ------------- this is where the previous identifier occurred
...
2692 | pub unsafe fn _mm_storel_pd(mem_addr: *mut f64, a: __m128d) {

error: identifier pair considered confusable between `e1` and `el`
    --> src/libcore/slice/mod.rs:2166:13
     |
     |
2166 |         for el in self {
     |             ^^
     | 
    ::: src/libcore/../stdarch/crates/core_arch/src/x86/sse2.rs:991:30
     |
991  | pub unsafe fn _mm_set_epi64x(e1: i64, e0: i64) -> __m128i {
     |                              -- this is where the previous identifier occurred
error: identifier pair considered confusable between `I` and `l`
    --> src/libcore/slice/mod.rs:5880:13
     |
     |
5880 |         let l = cmp::min(left.len(), right.len());
     | 
    ::: src/libcore/ptr/mod.rs:1382:45
     |
     |
1382 | fnptr_impls_args! { A, B, C, D, E, F, G, H, I }
     |                                             - this is where the previous identifier occurred

error: identifier pair considered confusable between `test_mm_load1_pd` and `test_mm_loadl_pd`
     |
4737 |     unsafe fn test_mm_loadl_pd() {
     |               ^^^^^^^^^^^^^^^^
...
...
5063 |     unsafe fn test_mm_load1_pd() {
     |               ---------------- this is where the previous identifier occurred

error: identifier pair considered confusable between `test_mm_store1_pd` and `test_mm_storel_pd`
     |
4805 |     unsafe fn test_mm_store1_pd() {
     |               ----------------- this is where the previous identifier occurred
...
---
  local time: Fri May  1 02:13:21 UTC 2020
  network time: Fri, 01 May 2020 02:13:21 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71542/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71542/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (6810) (python)
##[section]Finishing: Finalize Job
