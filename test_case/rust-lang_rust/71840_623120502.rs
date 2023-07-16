plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/118d3c05-6c8a-461d-8a7a-eeeaa2c804c1.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71840/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71840/merge:refs/remotes/pull/71840/merge
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
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
.................i.................................................................................. 1800/9973
.................................................................................................... 1900/9973
.................................i.................................................................. 2000/9973
.................................................................................................... 2100/9973
.......................iiiii........................................................................ 2200/9973
.................................................................................................... 2400/9973
.................................................................................................... 2500/9973
.................................................................................................... 2600/9973
.................................................................................................... 2700/9973
---
........i...............i........................................................................... 5100/9973
.................................................................................................... 5200/9973
......................................................i............................................. 5300/9973
.............................................i...................................................... 5400/9973
...............................................ii.ii........i...i................................... 5500/9973
..............................................................................................i..... 5700/9973
.................................................................................................... 5800/9973
..............................ii.....................................i.............................. 5900/9973
.................................................................................................... 6000/9973
.................................................................................................... 6000/9973
.................................................................................................... 6100/9973
................................................................ii...i..ii...........i.............. 6200/9973
.................................................................................................... 6400/9973
.................................................................................................... 6500/9973
................................................................................................i..i 6600/9973
i................................................................................................... 6700/9973
---
.................................................................................................... 7900/9973
.................................................................................................... 8000/9973
.................................................................................................... 8100/9973
........i........................................................................................... 8200/9973
.............................................................iiiiii.iiiii.i......................... 8300/9973
..............i..................................................................................... 8500/9973
.................................................................................................... 8600/9973
.................................................................................................... 8700/9973
.................................................................................................... 8800/9973
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 186 tests
iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......

 finished in 5.720
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.157
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 15.209
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2500 tests
......iiiii......................................................................................... 100/2500
......................................................................................ii............ 200/2500
......................i............................................................................. 400/2500
............................................................................i..i..................ii 500/2500
ii.................................................................................................. 600/2500
.................................................................................................... 700/2500
---
.................................................................................................... 500/764
......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
.......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
...........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs.:1997:22
...thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2022:17
.thread '<unnamed>' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError.', .src/libstd/sync/mpsc/mod.rs.:2034:21
..thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:634:13
..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:588:13
...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:564:13
.thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:695:13
---

running 1020 tests
i................................................................................................... 100/1020
.................................................................................................... 200/1020
...................iii......i......i...i......i..................................................... 300/1020
.................................................................................................... 400/1020
....................................................i....i......................................ii.. 500/1020
.................................................................................................... 700/1020
.................................................................................................... 700/1020
..............................................iiii.................................................. 800/1020
.................................................................................................... 900/1020
....................................................................iiii............................ 1000/1020
test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out

 finished in 160.913
Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 1.097
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
     Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-fb28a784e792762c

running 0 tests

---
Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 211 tests
......................i...ii.......................................................................i 100/211
........................................iiiiii......i..............iii.............................. 200/211
.......ii..

 finished in 72.264
Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
doc tests for: /checkout/src/doc/rustc/src/targets/index.md
doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
 finished in 4.215
Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
Checking "alias-1" ... OK
Checking "alias-2" ... OK
Checking "alias-3" ... OK
Checking "alias" ... OK
Checking "basic" ... OK
Checking "deduplication" ... OK
Checking "enum-option" ... OK
Checking "filter-crate" ... OK
Checking "fn-forget" ... OK
Checking "from_u" ... OK
Checking "keyword" ... OK
Checking "macro-check" ... OK
Checking "macro-print" ... OK
Checking "multi-query" ... OK
Checking "never" ... OK
Checking "quoted" ... OK
Checking "return-specific-literal" ... OK
Checking "return-specific" ... OK
Checking "should-fail" ... OK
Checking "string-from_ut" ... OK
Checking "struct-vec" ... OK
Checking "vec-new" ... OK
Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 6 tests
......
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 13 tests
.iiiiiii.iii.

 finished in 0.744
Build completed successfully in 1:47:12
Build completed successfully in 1:47:12
+ python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.34s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
running 96 tests
..................................................F.........F......F..F....F...................F
failures:

---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
17           StorageLive(_1);                 // scope 0 at $DIR/inline-into-box-place.rs:8:9: 8:11
18           StorageLive(_2);                 // scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43
19           _2 = Box(std::vec::Vec<u32>);    // scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43
- -         (*_2) = const std::vec::Vec::<u32>::new() -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
+ -         (*_2) = const std::vec::Vec::<u32>::new() -> [return: bb1, unwind: bb4]; // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
21 +         _4 = &mut (*_2);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
22 +         ((*_4).0: alloc::raw_vec::RawVec<u32>) = const alloc::raw_vec::RawVec::<u32> { ptr: std::ptr::Unique::<u32> { pointer: {0x4 as *const u32}, _marker: std::marker::PhantomData::<u32> }, cap: 0usize, alloc: std::alloc::Global }; // scope 2 at $SRC_DIR/liballoc/vec.rs:LL:COL
23                                            // ty::Const

31 -                                          // + literal: Const { ty: fn() -> std::vec::Vec<u32> {std::vec::Vec::<u32>::new}, val: Value(Scalar(<ZST>)) }
33 - 
33 - 
- -     bb1 (cleanup): {
- -         resume;                          // scope 0 at $DIR/inline-into-box-place.rs:7:1: 9:2
- - 
- -     bb2: {
+ -     bb1: {
+ -     bb1: {
39 +                                          // + span: $SRC_DIR/liballoc/vec.rs:LL:COL
40 +                                          // + user_ty: UserType(0)
41 +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
55                                            // mir::Constant
55                                            // mir::Constant
56                                            // + span: $DIR/inline-into-box-place.rs:7:11: 9:2
57                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
- -         drop(_1) -> [return: bb3, unwind: bb1]; // scope 0 at $DIR/inline-into-box-place.rs:9:1: 9:2
- +         drop(_1) -> [return: bb2, unwind: bb1]; // scope 0 at $DIR/inline-into-box-place.rs:9:1: 9:2
+ -         drop(_1) -> [return: bb2, unwind: bb3]; // scope 0 at $DIR/inline-into-box-place.rs:9:1: 9:2
+ +         drop(_1) -> [return: bb1, unwind: bb2]; // scope 0 at $DIR/inline-into-box-place.rs:9:1: 9:2
61   
- -     bb3: {
- -     bb3: {
- -         StorageDead(_1);                 // scope 0 at $DIR/inline-into-box-place.rs:9:1: 9:2
- -         return;                          // scope 0 at $DIR/inline-into-box-place.rs:9:2: 9:2
- +     bb1 (cleanup): {
- +         resume;                          // scope 0 at $DIR/inline-into-box-place.rs:7:1: 9:2
+ -     bb2: {
+ +     bb1: {
+           StorageDead(_1);                 // scope 0 at $DIR/inline-into-box-place.rs:9:1: 9:2
+           return;                          // scope 0 at $DIR/inline-into-box-place.rs:9:2: 9:2
68   
68   
+ -     bb3 (cleanup): {
+ +     bb2 (cleanup): {
+           resume;                          // scope 0 at $DIR/inline-into-box-place.rs:7:1: 9:2
+ -     }
+ - 
69 -     bb4 (cleanup): {
- -         _3 = const alloc::alloc::box_free::<std::vec::Vec<u32>>(move (_2.0: std::ptr::Unique<std::vec::Vec<u32>>)) -> bb1; // scope 0 at $DIR/inline-into-box-place.rs:8:42: 8:43
+ -         _3 = const alloc::alloc::box_free::<std::vec::Vec<u32>>(move (_2.0: std::ptr::Unique<std::vec::Vec<u32>>)) -> bb3; // scope 0 at $DIR/inline-into-box-place.rs:8:42: 8:43
71 -                                          // ty::Const
72 -                                          // + ty: unsafe fn(std::ptr::Unique<std::vec::Vec<u32>>) {alloc::alloc::box_free::<std::vec::Vec<u32>>}
73 -                                          // + val: Value(Scalar(<ZST>))
74 -                                          // mir::Constant
74 -                                          // mir::Constant
75 -                                          // + span: $DIR/inline-into-box-place.rs:8:42: 8:43
76 -                                          // + literal: Const { ty: unsafe fn(std::ptr::Unique<std::vec::Vec<u32>>) {alloc::alloc::box_free::<std::vec::Vec<u32>>}, val: Value(Scalar(<ZST>)) }
- +     bb2: {
- +         StorageDead(_1);                 // scope 0 at $DIR/inline-into-box-place.rs:9:1: 9:2
- +         return;                          // scope 0 at $DIR/inline-into-box-place.rs:9:2: 9:2
81   }
82   


thread '[mir-opt] mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline-into-box-place/32bit/rustc.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3166:25

---- [mir-opt] mir-opt/issue-41697.rs stdout ----
18                                          // mir::Constant
18                                          // mir::Constant
19                                          // + span: $DIR/issue-41697.rs:18:21: 18:22
20                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000001)) }
-         assert(!move (_1.1: bool), "attempt to add with overflow") -> [success: bb2, unwind: bb1]; // scope 0 at $DIR/issue-41697.rs:18:19: 18:22
+         assert(!move (_1.1: bool), "attempt to add with overflow") -> [success: bb1, unwind: bb2]; // scope 0 at $DIR/issue-41697.rs:18:19: 18:22
23 
23 
-     bb1 (cleanup): {
-         resume;                          // scope 0 at $DIR/issue-41697.rs:18:19: 18:22
- 
-     bb2: {
+     bb1: {
+     bb1: {
29         _0 = move (_1.0: usize);         // scope 0 at $DIR/issue-41697.rs:18:19: 18:22
30         return;                          // scope 0 at $DIR/issue-41697.rs:18:19: 18:22
+ 
+ 
+     bb2 (cleanup): {
+         resume;                          // scope 0 at $DIR/issue-41697.rs:18:19: 18:22
32 }
33 


thread '[mir-opt] mir-opt/issue-41697.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue-41697/32bit/rustc.{{impl}}-{{constant}}.SimplifyCfg-qualify-consts.after.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
5 | '_#1r | Local | ['_#1r]
7 | Inferred Region Values
7 | Inferred Region Values
- | '_#0r | U0 | {bb0[0..=8], bb1[0], bb2[0..=8], bb3[0], bb4[0..=1], bb5[0..=3], bb6[0..=3], bb7[0..=2], bb8[0..=5], '_#0r, '_#1r}
- | '_#1r | U0 | {bb0[0..=8], bb1[0], bb2[0..=8], bb3[0], bb4[0..=1], bb5[0..=3], bb6[0..=3], bb7[0..=2], bb8[0..=5], '_#1r}
+ | '_#0r | U0 | {bb0[0..=8], bb1[0..=8], bb2[0], bb3[0..=1], bb4[0..=3], bb5[0..=3], bb6[0..=2], bb7[0..=5], bb8[0], '_#0r, '_#1r}
+ | '_#1r | U0 | {bb0[0..=8], bb1[0..=8], bb2[0], bb3[0..=1], bb4[0..=3], bb5[0..=3], bb6[0..=2], bb7[0..=5], bb8[0], '_#1r}
10 | '_#2r | U0 | {}
- | '_#3r | U0 | {bb2[0..=8], bb3[0], bb5[0..=2]}
- | '_#4r | U0 | {bb2[1..=8], bb3[0], bb5[0..=2]}
- | '_#5r | U0 | {bb2[4..=8], bb3[0], bb5[0..=2]}
+ | '_#3r | U0 | {bb1[0..=8], bb2[0], bb4[0..=2]}
+ | '_#4r | U0 | {bb1[1..=8], bb2[0], bb4[0..=2]}
+ | '_#5r | U0 | {bb1[4..=8], bb2[0], bb4[0..=2]}
15 | Inference Constraints
15 | Inference Constraints
- | '_#0r live at {bb0[0..=8], bb1[0], bb2[0..=8], bb3[0], bb4[0..=1], bb5[0..=3], bb6[0..=3], bb7[0..=2], bb8[0..=5]}
- | '_#1r live at {bb0[0..=8], bb1[0], bb2[0..=8], bb3[0], bb4[0..=1], bb5[0..=3], bb6[0..=3], bb7[0..=2], bb8[0..=5]}
- | '_#3r live at {bb2[0]}
- | '_#4r live at {bb2[1..=3]}
- | '_#5r live at {bb2[4..=8], bb3[0], bb5[0..=2]}
- | '_#3r: '_#4r due to Assignment at Single(bb2[0])
- | '_#4r: '_#5r due to Assignment at Single(bb2[3])
+ | '_#0r live at {bb0[0..=8], bb1[0..=8], bb2[0], bb3[0..=1], bb4[0..=3], bb5[0..=3], bb6[0..=2], bb7[0..=5], bb8[0]}
+ | '_#1r live at {bb0[0..=8], bb1[0..=8], bb2[0], bb3[0..=1], bb4[0..=3], bb5[0..=3], bb6[0..=2], bb7[0..=5], bb8[0]}
+ | '_#3r live at {bb1[0]}
+ | '_#4r live at {bb1[1..=3]}
+ | '_#5r live at {bb1[4..=8], bb2[0], bb4[0..=2]}
+ | '_#3r: '_#4r due to Assignment at Single(bb1[0])
+ | '_#4r: '_#5r due to Assignment at Single(bb1[3])
24 fn main() -> () {
25     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:16:11: 16:11


76                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000000)) }
77         _4 = Len(_1);                    // bb0[6]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18
78         _5 = Lt(_3, _4);                 // bb0[7]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18
-         assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> [success: bb2, unwind: bb1]; // bb0[8]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18
+         assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> [success: bb1, unwind: bb8]; // bb0[8]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18
81 
81 
-     bb1 (cleanup): {
-         resume;                          // bb1[0]: scope 0 at $DIR/region-subtyping-basic.rs:16:1: 25:2
- 
-     bb2: {
-     bb2: {
-         _2 = &'_#3r _1[_3];              // bb2[0]: scope 1 at $DIR/region-subtyping-basic.rs:18:13: 18:18
-         FakeRead(ForLet, _2);            // bb2[1]: scope 1 at $DIR/region-subtyping-basic.rs:18:9: 18:10
-         StorageLive(_6);                 // bb2[2]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
-         _6 = _2;                         // bb2[3]: scope 2 at $DIR/region-subtyping-basic.rs:19:13: 19:14
-         FakeRead(ForLet, _6);            // bb2[4]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
-         StorageLive(_7);                 // bb2[5]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
-         _7 = const Const(Value(Scalar(0x01)): bool); // bb2[6]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
+     bb1: {
+         _2 = &'_#3r _1[_3];              // bb1[0]: scope 1 at $DIR/region-subtyping-basic.rs:18:13: 18:18
+         FakeRead(ForLet, _2);            // bb1[1]: scope 1 at $DIR/region-subtyping-basic.rs:18:9: 18:10
+         StorageLive(_6);                 // bb1[2]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
+         _6 = _2;                         // bb1[3]: scope 2 at $DIR/region-subtyping-basic.rs:19:13: 19:14
+         FakeRead(ForLet, _6);            // bb1[4]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
+         StorageLive(_7);                 // bb1[5]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
+         _7 = const Const(Value(Scalar(0x01)): bool); // bb1[6]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
94                                          // ty::Const
95                                          // + ty: bool
96                                          // + val: Value(Scalar(0x01))
97                                          // mir::Constant
97                                          // mir::Constant
98                                          // + span: $DIR/region-subtyping-basic.rs:20:8: 20:12
99                                          // + literal: Const { ty: bool, val: Value(Scalar(0x01)) }
-         FakeRead(ForMatchedPlace, _7);   // bb2[7]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
-         switchInt(_7) -> [Const(Value(Scalar(0x00)): bool): bb4, otherwise: bb3]; // bb2[8]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
+         FakeRead(ForMatchedPlace, _7);   // bb1[7]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
+         switchInt(_7) -> [Const(Value(Scalar(0x00)): bool): bb3, otherwise: bb2]; // bb1[8]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
103 
-     bb3: {
-     bb3: {
-         falseEdges -> [real: bb5, imaginary: bb4]; // bb3[0]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
+     bb2: {
+         falseEdges -> [real: bb4, imaginary: bb3]; // bb2[0]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
107 
-     bb4: {
-     bb4: {
-         StorageLive(_10);                // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
-         _10 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(const Const(Value(Scalar(0x00000016)): usize)) -> [return: bb7, unwind: bb1]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
+     bb3: {
+         StorageLive(_10);                // bb3[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
+         _10 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(const Const(Value(Scalar(0x00000016)): usize)) -> [return: bb6, unwind: bb8]; // bb3[1]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
111                                          // ty::Const
112                                          // + ty: fn(usize) -> bool {use_x}
113                                          // + val: Value(Scalar(<ZST>))

122                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000016)) }
124 
-     bb5: {
-     bb5: {
-         StorageLive(_8);                 // bb5[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
-         StorageLive(_9);                 // bb5[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
-         _9 = (*_6);                      // bb5[2]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
-         _8 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(move _9) -> [return: bb6, unwind: bb1]; // bb5[3]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
+     bb4: {
+         StorageLive(_8);                 // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
+         StorageLive(_9);                 // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
+         _9 = (*_6);                      // bb4[2]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
+         _8 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(move _9) -> [return: bb5, unwind: bb8]; // bb4[3]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
130                                          // ty::Const
131                                          // + ty: fn(usize) -> bool {use_x}
132                                          // + val: Value(Scalar(<ZST>))

135                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
137 
-     bb6: {
-     bb6: {
-         StorageDead(_9);                 // bb6[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:17: 21:18
-         StorageDead(_8);                 // bb6[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:18: 21:19
-         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb6[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:13: 22:6
+     bb5: {
+         StorageDead(_9);                 // bb5[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:17: 21:18
+         StorageDead(_8);                 // bb5[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:18: 21:19
+         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb5[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:13: 22:6
142                                          // ty::Const
143                                          // + ty: ()
144                                          // + val: Value(Scalar(<ZST>))
145                                          // mir::Constant
145                                          // mir::Constant
146                                          // + span: $DIR/region-subtyping-basic.rs:20:13: 22:6
147                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
-         goto -> bb8;                     // bb6[3]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
+         goto -> bb7;                     // bb5[3]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
150 
-     bb7: {
-     bb7: {
-         StorageDead(_10);                // bb7[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:18: 23:19
-         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb7[1]: scope 3 at $DIR/region-subtyping-basic.rs:22:12: 24:6
+     bb6: {
+         StorageDead(_10);                // bb6[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:18: 23:19
+         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb6[1]: scope 3 at $DIR/region-subtyping-basic.rs:22:12: 24:6
154                                          // ty::Const
155                                          // + ty: ()
156                                          // + val: Value(Scalar(<ZST>))
157                                          // mir::Constant
157                                          // mir::Constant
158                                          // + span: $DIR/region-subtyping-basic.rs:22:12: 24:6
159                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
-         goto -> bb8;                     // bb7[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
+         goto -> bb7;                     // bb6[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
162 
-     bb8: {
-     bb8: {
-         StorageDead(_6);                 // bb8[0]: scope 2 at $DIR/region-subtyping-basic.rs:25:1: 25:2
-         StorageDead(_3);                 // bb8[1]: scope 1 at $DIR/region-subtyping-basic.rs:25:1: 25:2
-         StorageDead(_2);                 // bb8[2]: scope 1 at $DIR/region-subtyping-basic.rs:25:1: 25:2
-         StorageDead(_1);                 // bb8[3]: scope 0 at $DIR/region-subtyping-basic.rs:25:1: 25:2
-         StorageDead(_7);                 // bb8[4]: scope 0 at $DIR/region-subtyping-basic.rs:25:1: 25:2
-         return;                          // bb8[5]: scope 0 at $DIR/region-subtyping-basic.rs:25:2: 25:2
+     bb7: {
+         StorageDead(_6);                 // bb7[0]: scope 2 at $DIR/region-subtyping-basic.rs:25:1: 25:2
+         StorageDead(_3);                 // bb7[1]: scope 1 at $DIR/region-subtyping-basic.rs:25:1: 25:2
+         StorageDead(_2);                 // bb7[2]: scope 1 at $DIR/region-subtyping-basic.rs:25:1: 25:2
+         StorageDead(_1);                 // bb7[3]: scope 0 at $DIR/region-subtyping-basic.rs:25:1: 25:2
+         StorageDead(_7);                 // bb7[4]: scope 0 at $DIR/region-subtyping-basic.rs:25:1: 25:2
+         return;                          // bb7[5]: scope 0 at $DIR/region-subtyping-basic.rs:25:2: 25:2
+ 
+ 
+     bb8 (cleanup): {
+         resume;                          // bb8[0]: scope 0 at $DIR/region-subtyping-basic.rs:16:1: 25:2
171 }
172 


thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region-subtyping-basic/32bit/rustc.main.nll.0.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/packed-struct-drop-aligned.rs stdout ----
---- [mir-opt] mir-opt/packed-struct-drop-aligned.rs stdout ----
43         drop(_6) -> [return: bb4, unwind: bb3]; // scope 1 at $DIR/packed-struct-drop-aligned.rs:7:5: 7:8
45 
45 
-     bb1 (cleanup): {
-         resume;                          // scope 0 at $DIR/packed-struct-drop-aligned.rs:5:1: 8:2
- 
-     bb2: {
+     bb1: {
+     bb1: {
51         StorageDead(_1);                 // scope 0 at $DIR/packed-struct-drop-aligned.rs:8:1: 8:2
52         return;                          // scope 0 at $DIR/packed-struct-drop-aligned.rs:8:2: 8:2

54 
54 
+     bb2 (cleanup): {
+         resume;                          // scope 0 at $DIR/packed-struct-drop-aligned.rs:5:1: 8:2
+ 
+ 
55     bb3 (cleanup): {
56         (_1.0: Aligned) = move _4;       // scope 1 at $DIR/packed-struct-drop-aligned.rs:7:5: 7:8
-         drop(_1) -> bb1;                 // scope 0 at $DIR/packed-struct-drop-aligned.rs:8:1: 8:2
+         drop(_1) -> bb2;                 // scope 0 at $DIR/packed-struct-drop-aligned.rs:8:1: 8:2
59 
60     bb4: {

68                                          // mir::Constant
68                                          // mir::Constant
69                                          // + span: $DIR/packed-struct-drop-aligned.rs:5:11: 8:2
70                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
-         drop(_1) -> [return: bb2, unwind: bb1]; // scope 0 at $DIR/packed-struct-drop-aligned.rs:8:1: 8:2
+         drop(_1) -> [return: bb1, unwind: bb2]; // scope 0 at $DIR/packed-struct-drop-aligned.rs:8:1: 8:2
73 }
74 


thread '[mir-opt] mir-opt/packed-struct-drop-aligned.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/packed-struct-drop-aligned/32bit/rustc.main.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/simple-match.rs stdout ----
6 
7     bb0: {
7     bb0: {
8         FakeRead(ForMatchedPlace, _1);   // scope 0 at $DIR/simple-match.rs:6:11: 6:12
-         switchInt(_1) -> [false: bb3, otherwise: bb2]; // scope 0 at $DIR/simple-match.rs:7:9: 7:13
+         switchInt(_1) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/simple-match.rs:7:9: 7:13
11 
11 
-     bb1 (cleanup): {
-         resume;                          // scope 0 at $DIR/simple-match.rs:5:1: 10:2
+     bb1: {
+         falseEdges -> [real: bb3, imaginary: bb2]; // scope 0 at $DIR/simple-match.rs:7:9: 7:13
15 
16     bb2: {


-         falseEdges -> [real: bb4, imaginary: bb3]; // scope 0 at $DIR/simple-match.rs:7:9: 7:13
- 
-     bb3: {
21         _0 = const 20usize;              // scope 0 at $DIR/simple-match.rs:8:14: 8:16
22                                          // ty::Const
22                                          // ty::Const
23                                          // + ty: usize

25                                          // mir::Constant
26                                          // + span: $DIR/simple-match.rs:8:14: 8:16
27                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000014)) }
-         goto -> bb5;                     // scope 0 at $DIR/simple-match.rs:6:5: 9:6
+         goto -> bb4;                     // scope 0 at $DIR/simple-match.rs:6:5: 9:6
30 
-     bb4: {
+     bb3: {
32         _0 = const 10usize;              // scope 0 at $DIR/simple-match.rs:7:17: 7:19
32         _0 = const 10usize;              // scope 0 at $DIR/simple-match.rs:7:17: 7:19
33                                          // ty::Const
34                                          // + ty: usize

36                                          // mir::Constant
37                                          // + span: $DIR/simple-match.rs:7:17: 7:19
38                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000a)) }
-         goto -> bb5;                     // scope 0 at $DIR/simple-match.rs:6:5: 9:6
+         goto -> bb4;                     // scope 0 at $DIR/simple-match.rs:6:5: 9:6
41 
-     bb5: {
-     bb5: {
-         goto -> bb6;                     // scope 0 at $DIR/simple-match.rs:10:2: 10:2
- 
-     bb6: {
+     bb4: {
47         return;                          // scope 0 at $DIR/simple-match.rs:10:2: 10:2
47         return;                          // scope 0 at $DIR/simple-match.rs:10:2: 10:2
48     }
49 }

thread '[mir-opt] mir-opt/simple-match.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simple-match/32bit/rustc.match_bool.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
13                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000002)) }
14         return;                          // scope 0 at $DIR/unusual-item-types.rs:10:5: 10:40
- 
- 
-     bb1 (cleanup): {
-         resume;                          // scope 0 at $DIR/unusual-item-types.rs:10:5: 10:40
20 }
21 


thread '[mir-opt] mir-opt/unusual-item-types.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unusual-item-types/32bit/rustc.{{impl}}-ASSOCIATED_CONSTANT.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3166:25

failures:
    [mir-opt] mir-opt/inline/inline-into-box-place.rs
    [mir-opt] mir-opt/issue-41697.rs
---
test result: FAILED. 90 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"



failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
== clock drift check ==
  local time: Sun May  3 14:41:15 UTC 2020
  network time: Sun, 03 May 2020 14:41:15 GMT
== end clock drift check ==
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71840/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71840/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4659) (python)
##[section]Finishing: Finalize Job
