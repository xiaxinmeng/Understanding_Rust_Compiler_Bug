plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
................................i................................
failures:

---- [mir-opt] mir-opt/const_prop/boxes.rs stdout ----
14           debug x => _1;                   // in scope 1 at $DIR/boxes.rs:12:9: 12:10
16       scope 2 {
16       scope 2 {
+           scope 3 (inlined alloc::alloc::exchange_malloc) { // at $DIR/boxes.rs:12:14: 12:22
+               debug size => _4;            // in scope 3 at $DIR/boxes.rs:12:14: 12:22
+               debug align => _5;           // in scope 3 at $DIR/boxes.rs:12:14: 12:22
+               let _8: std::alloc::Layout;  // in scope 3 at $DIR/boxes.rs:12:14: 12:22
+               let mut _9: usize;           // in scope 3 at $DIR/boxes.rs:12:14: 12:22
+               let mut _10: usize;          // in scope 3 at $DIR/boxes.rs:12:14: 12:22
+               let mut _11: std::result::Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError>; // in scope 3 at $DIR/boxes.rs:12:14: 12:22
+               let mut _12: &std::alloc::Global; // in scope 3 at $DIR/boxes.rs:12:14: 12:22
+               let mut _13: std::alloc::Layout; // in scope 3 at $DIR/boxes.rs:12:14: 12:22
+               let mut _14: isize;          // in scope 3 at $DIR/boxes.rs:12:14: 12:22
+               let mut _16: std::ptr::NonNull<[u8]>; // in scope 3 at $DIR/boxes.rs:12:14: 12:22
+               let mut _17: std::alloc::Layout; // in scope 3 at $DIR/boxes.rs:12:14: 12:22
+               scope 4 {
+                   debug layout => _8;      // in scope 4 at $DIR/boxes.rs:12:14: 12:22
+                   let _15: std::ptr::NonNull<[u8]>; // in scope 4 at $DIR/boxes.rs:12:14: 12:22
+                   let mut _18: &std::alloc::Global; // in scope 4 at $DIR/boxes.rs:12:14: 12:22
+                   scope 6 {
+                       debug ptr => _15;    // in scope 6 at $DIR/boxes.rs:12:14: 12:22
+                       scope 12 (inlined NonNull::<[u8]>::as_mut_ptr) { // at $DIR/boxes.rs:12:14: 12:22
+                           debug self => _16; // in scope 12 at $DIR/boxes.rs:12:14: 12:22
+                           let mut _25: std::ptr::NonNull<u8>; // in scope 12 at $DIR/boxes.rs:12:14: 12:22
+                           let mut _26: std::ptr::NonNull<[u8]>; // in scope 12 at $DIR/boxes.rs:12:14: 12:22
+                           scope 13 (inlined NonNull::<[u8]>::as_non_null_ptr) { // at $DIR/boxes.rs:12:14: 12:22
+                               debug self => _26; // in scope 13 at $DIR/boxes.rs:12:14: 12:22
+                               let mut _27: *mut u8; // in scope 13 at $DIR/boxes.rs:12:14: 12:22
+                               let mut _28: *mut [u8]; // in scope 13 at $DIR/boxes.rs:12:14: 12:22
+                               let mut _29: std::ptr::NonNull<[u8]>; // in scope 13 at $DIR/boxes.rs:12:14: 12:22
+                               scope 14 {
+                                   scope 15 (inlined NonNull::<[u8]>::as_ptr) { // at $DIR/boxes.rs:12:14: 12:22
+                                       debug self => _29; // in scope 15 at $DIR/boxes.rs:12:14: 12:22
+                                       let mut _30: *const [u8]; // in scope 15 at $DIR/boxes.rs:12:14: 12:22
+                                   }
+                                   scope 16 (inlined ptr::mut_ptr::<impl *mut [u8]>::as_mut_ptr) { // at $DIR/boxes.rs:12:14: 12:22
+                                       debug self => _28; // in scope 16 at $DIR/boxes.rs:12:14: 12:22
+                                       let mut _31: *mut [u8]; // in scope 16 at $DIR/boxes.rs:12:14: 12:22
+                                   }
+                                   scope 17 (inlined NonNull::<u8>::new_unchecked) { // at $DIR/boxes.rs:12:14: 12:22
+                                       debug ptr => _27; // in scope 17 at $DIR/boxes.rs:12:14: 12:22
+                                       let mut _32: *const u8; // in scope 17 at $DIR/boxes.rs:12:14: 12:22
+                                       let mut _33: *const u8; // in scope 17 at $DIR/boxes.rs:12:14: 12:22
+                                       let mut _34: *mut u8; // in scope 17 at $DIR/boxes.rs:12:14: 12:22
+                                       scope 18 {
+                                   }
+                               }
+                           }
+                           }
+                           scope 19 (inlined NonNull::<u8>::as_ptr) { // at $DIR/boxes.rs:12:14: 12:22
+                               debug self => _25; // in scope 19 at $DIR/boxes.rs:12:14: 12:22
+                               let mut _35: *const u8; // in scope 19 at $DIR/boxes.rs:12:14: 12:22
+                       }
+                   }
+                   }
+                   scope 11 (inlined <std::alloc::Global as Allocator>::allocate) { // at $DIR/boxes.rs:12:14: 12:22
+                       debug self => _12;   // in scope 11 at $DIR/boxes.rs:12:14: 12:22
+                       debug layout => _13; // in scope 11 at $DIR/boxes.rs:12:14: 12:22
+                       let mut _23: &std::alloc::Global; // in scope 11 at $DIR/boxes.rs:12:14: 12:22
+                       let mut _24: std::alloc::Layout; // in scope 11 at $DIR/boxes.rs:12:14: 12:22
+               }
+               scope 5 {
+               scope 5 {
+                   scope 7 (inlined Layout::from_size_align_unchecked) { // at $DIR/boxes.rs:12:14: 12:22
+                       debug size => _9;    // in scope 7 at $DIR/boxes.rs:12:14: 12:22
+                       debug align => _10;  // in scope 7 at $DIR/boxes.rs:12:14: 12:22
+                       let mut _19: usize;  // in scope 7 at $DIR/boxes.rs:12:14: 12:22
+                       let mut _20: std::num::NonZeroUsize; // in scope 7 at $DIR/boxes.rs:12:14: 12:22
+                       let mut _21: usize;  // in scope 7 at $DIR/boxes.rs:12:14: 12:22
+                       scope 8 {
+                           scope 9 (inlined NonZeroUsize::new_unchecked) { // at $DIR/boxes.rs:12:14: 12:22
+                               debug n => _21; // in scope 9 at $DIR/boxes.rs:12:14: 12:22
+                               let mut _22: usize; // in scope 9 at $DIR/boxes.rs:12:14: 12:22
+                               scope 10 {
+                           }
+                       }
+                   }
+               }
+               }
+           }
17       }
18   
19       bb0: {

22           StorageLive(_3);                 // scope 0 at $DIR/boxes.rs:12:14: 12:22
23 -         _4 = SizeOf(i32);                // scope 2 at $DIR/boxes.rs:12:14: 12:22
24 -         _5 = AlignOf(i32);               // scope 2 at $DIR/boxes.rs:12:14: 12:22
- -         _6 = alloc::alloc::exchange_malloc(move _4, move _5) -> bb1; // scope 2 at $DIR/boxes.rs:12:14: 12:22
26 +         _4 = const 4_usize;              // scope 2 at $DIR/boxes.rs:12:14: 12:22
27 +         _5 = const 4_usize;              // scope 2 at $DIR/boxes.rs:12:14: 12:22
- +         _6 = alloc::alloc::exchange_malloc(const 4_usize, const 4_usize) -> bb1; // scope 2 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_14);                // scope 2 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_18);                // scope 2 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_8);                 // scope 3 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_9);                 // scope 5 at $DIR/boxes.rs:12:14: 12:22
+ -         _9 = _4;                         // scope 5 at $DIR/boxes.rs:12:14: 12:22
+ +         _9 = const 4_usize;              // scope 5 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_10);                // scope 5 at $DIR/boxes.rs:12:14: 12:22
+ -         _10 = _5;                        // scope 5 at $DIR/boxes.rs:12:14: 12:22
+ +         _10 = const 4_usize;             // scope 5 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_19);                // scope 7 at $DIR/boxes.rs:12:14: 12:22
+ -         _19 = _9;                        // scope 7 at $DIR/boxes.rs:12:14: 12:22
+ +         _19 = const 4_usize;             // scope 7 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_20);                // scope 7 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_21);                // scope 8 at $DIR/boxes.rs:12:14: 12:22
+ -         _21 = _10;                       // scope 8 at $DIR/boxes.rs:12:14: 12:22
+ +         _21 = const 4_usize;             // scope 8 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_22);                // scope 10 at $DIR/boxes.rs:12:14: 12:22
+ -         _22 = _21;                       // scope 10 at $DIR/boxes.rs:12:14: 12:22
+ -         (_20.0: usize) = move _22;       // scope 10 at $DIR/boxes.rs:12:14: 12:22
+ +         _22 = const 4_usize;             // scope 10 at $DIR/boxes.rs:12:14: 12:22
+ +         (_20.0: usize) = const 4_usize;  // scope 10 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_22);                // scope 10 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_21);                // scope 8 at $DIR/boxes.rs:12:14: 12:22
+ -         (_8.0: usize) = move _19;        // scope 7 at $DIR/boxes.rs:12:14: 12:22
+ -         (_8.1: std::num::NonZeroUsize) = move _20; // scope 7 at $DIR/boxes.rs:12:14: 12:22
+ +         (_8.0: usize) = const 4_usize;   // scope 7 at $DIR/boxes.rs:12:14: 12:22
+ +         (_8.1: std::num::NonZeroUsize) = const NonZeroUsize(4_usize); // scope 7 at $DIR/boxes.rs:12:14: 12:22
+ +                                          // ty::Const
+ +                                          // + ty: std::num::NonZeroUsize
+ +                                          // + val: Value(Scalar(0x0000000000000004))
+ +                                          // mir::Constant
+ +                                          // + span: $DIR/boxes.rs:12:14: 12:22
+ +                                          // + literal: Const { ty: std::num::NonZeroUsize, val: Value(Scalar(0x0000000000000004)) }
+           StorageDead(_20);                // scope 7 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_19);                // scope 7 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_10);                // scope 5 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_9);                 // scope 5 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_11);                // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_12);                // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           _18 = const {alloc2: &std::alloc::Global}; // scope 4 at $DIR/boxes.rs:12:14: 12:22
+                                            // ty::Const
+                                            // + ty: &std::alloc::Global
+                                            // + val: Value(Scalar(alloc2))
29                                            // mir::Constant
30                                            // + span: $DIR/boxes.rs:12:14: 12:22
-                                            // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: &std::alloc::Global, val: Value(Scalar(alloc2)) }
+           _12 = _18;                       // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_13);                // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           _13 = _8;                        // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_23);                // scope 11 at $DIR/boxes.rs:12:14: 12:22
+           _23 = _12;                       // scope 11 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_24);                // scope 11 at $DIR/boxes.rs:12:14: 12:22
+           _24 = _13;                       // scope 11 at $DIR/boxes.rs:12:14: 12:22
+           _11 = std::alloc::Global::alloc_impl(move _23, move _24, const false) -> bb6; // scope 11 at $DIR/boxes.rs:12:14: 12:22
+                                            // mir::Constant
+                                            // + span: $DIR/boxes.rs:12:14: 12:22
+                                            // + literal: Const { ty: for<'r> fn(&'r std::alloc::Global, std::alloc::Layout, bool) -> std::result::Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {std::alloc::Global::alloc_impl}, val: Value(Scalar(<ZST>)) }
33   
34       bb1: {


+           StorageDead(_3);                 // scope 0 at $DIR/boxes.rs:12:26: 12:27
+           nop;                             // scope 0 at $DIR/boxes.rs:11:11: 13:2
+           StorageDead(_1);                 // scope 0 at $DIR/boxes.rs:13:1: 13:2
+           return;                          // scope 0 at $DIR/boxes.rs:13:2: 13:2
+   
+   
+       bb2 (cleanup): {
+           resume;                          // scope 0 at $DIR/boxes.rs:11:1: 13:2
+   
+       bb3: {
+       bb3: {
+           StorageLive(_17);                // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           _17 = _8;                        // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           handle_alloc_error(move _17);    // scope 4 at $DIR/boxes.rs:12:14: 12:22
+                                            // mir::Constant
+                                            // + span: $DIR/boxes.rs:12:14: 12:22
+                                            // + literal: Const { ty: fn(std::alloc::Layout) -> ! {std::alloc::handle_alloc_error}, val: Value(Scalar(<ZST>)) }
+   
+       bb4: {
+       bb4: {
+           unreachable;                     // scope 4 at $DIR/boxes.rs:12:14: 12:22
+   
+       bb5: {
+       bb5: {
+           StorageLive(_15);                // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           _15 = ((_11 as Ok).0: std::ptr::NonNull<[u8]>); // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_16);                // scope 6 at $DIR/boxes.rs:12:14: 12:22
+           _16 = _15;                       // scope 6 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_25);                // scope 12 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_26);                // scope 12 at $DIR/boxes.rs:12:14: 12:22
+           _26 = _16;                       // scope 12 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_27);                // scope 14 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_28);                // scope 14 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_29);                // scope 14 at $DIR/boxes.rs:12:14: 12:22
+           _29 = _26;                       // scope 14 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_30);                // scope 15 at $DIR/boxes.rs:12:14: 12:22
+           _30 = (_29.0: *const [u8]);      // scope 15 at $DIR/boxes.rs:12:14: 12:22
+           _28 = move _30 as *mut [u8] (Misc); // scope 15 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_30);                // scope 15 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_29);                // scope 14 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_31);                // scope 16 at $DIR/boxes.rs:12:14: 12:22
+           _31 = _28;                       // scope 16 at $DIR/boxes.rs:12:14: 12:22
+           _27 = move _31 as *mut u8 (Misc); // scope 16 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_31);                // scope 16 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_28);                // scope 14 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_32);                // scope 18 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_33);                // scope 18 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_34);                // scope 18 at $DIR/boxes.rs:12:14: 12:22
+           _34 = _27;                       // scope 18 at $DIR/boxes.rs:12:14: 12:22
+           _33 = move _34 as *const u8 (Pointer(MutToConstPointer)); // scope 18 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_34);                // scope 18 at $DIR/boxes.rs:12:14: 12:22
+           _32 = _33;                       // scope 18 at $DIR/boxes.rs:12:14: 12:22
+           (_25.0: *const u8) = move _32;   // scope 18 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_32);                // scope 18 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_33);                // scope 17 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_27);                // scope 14 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_26);                // scope 12 at $DIR/boxes.rs:12:14: 12:22
+           StorageLive(_35);                // scope 19 at $DIR/boxes.rs:12:14: 12:22
+           _35 = (_25.0: *const u8);        // scope 19 at $DIR/boxes.rs:12:14: 12:22
+           _6 = move _35 as *mut u8 (Misc); // scope 19 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_35);                // scope 19 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_25);                // scope 12 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_16);                // scope 6 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_15);                // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_8);                 // scope 3 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_11);                // scope 3 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_18);                // scope 2 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_14);                // scope 2 at $DIR/boxes.rs:12:14: 12:22
35           StorageLive(_7);                 // scope 0 at $DIR/boxes.rs:12:14: 12:22
36           _7 = ShallowInitBox(move _6, i32); // scope 0 at $DIR/boxes.rs:12:14: 12:22
37           (*_7) = const 42_i32;            // scope 0 at $DIR/boxes.rs:12:19: 12:21

40           _2 = (*_3);                      // scope 0 at $DIR/boxes.rs:12:13: 12:22
41           _1 = Add(move _2, const 0_i32);  // scope 0 at $DIR/boxes.rs:12:13: 12:26
42           StorageDead(_2);                 // scope 0 at $DIR/boxes.rs:12:25: 12:26
-           drop(_3) -> [return: bb2, unwind: bb3]; // scope 0 at $DIR/boxes.rs:12:26: 12:27
+           drop(_3) -> [return: bb1, unwind: bb2]; // scope 0 at $DIR/boxes.rs:12:26: 12:27
45   
-       bb2: {
-       bb2: {
-           StorageDead(_3);                 // scope 0 at $DIR/boxes.rs:12:26: 12:27
-           nop;                             // scope 0 at $DIR/boxes.rs:11:11: 13:2
-           StorageDead(_1);                 // scope 0 at $DIR/boxes.rs:13:1: 13:2
-           return;                          // scope 0 at $DIR/boxes.rs:13:2: 13:2
+       bb6: {
+           StorageDead(_24);                // scope 11 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_23);                // scope 11 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_13);                // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           StorageDead(_12);                // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           _14 = discriminant(_11);         // scope 4 at $DIR/boxes.rs:12:14: 12:22
+           switchInt(move _14) -> [0_isize: bb5, 1_isize: bb3, otherwise: bb4]; // scope 4 at $DIR/boxes.rs:12:14: 12:22
-   
-   
-       bb3 (cleanup): {
-           resume;                          // scope 0 at $DIR/boxes.rs:11:1: 13:2
56   }
+   
+   
+   alloc2 (size: 0, align: 1) {}
58 


thread '[mir-opt] mir-opt/const_prop/boxes.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/boxes.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3578:25


failures:
    [mir-opt] mir-opt/const_prop/boxes.rs
    [mir-opt] mir-opt/const_prop/boxes.rs

test result: FAILED. 161 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 4.34s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:05
