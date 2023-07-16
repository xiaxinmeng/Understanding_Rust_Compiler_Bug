diff
--- rustc_codegen_llvm.back-write-report_inline_asm.005-010.SimplifyBranchSame.before.mir
+++ rustc_codegen_llvm.back-write-report_inline_asm.005-010.SimplifyBranchSame.after.mir
@@ -1,11 +1,11 @@
-// MIR for `report_inline_asm` before SimplifyBranchSame
+// MIR for `report_inline_asm` after SimplifyBranchSame
 
 fn report_inline_asm(_1: &CodegenContext<LlvmCodegenBackend>, _2: std::string::String, _3: llvm_::ffi::DiagnosticLevel, _4: u32, _5: Option<(std::string::String, Vec<InnerSpan>)>) -> () {
     debug cgcx => _1;                    // in scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:244:5: 244:9
     debug msg => _2;                     // in scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:245:5: 245:8
     debug level => _3;                   // in scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:246:5: 246:10
     debug cookie => _4;                  // in scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:247:5: 247:15
     debug source => _5;                  // in scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:248:5: 248:11
     let mut _0: ();                      // return place in scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:249:3: 249:3
     let mut _6: isize;                   // in scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:253:27: 253:35
     let _7: rustc_errors::Level;         // in scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:256:9: 256:14
@@ -15,80 +15,76 @@
     let mut _11: u32;                    // in scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:261:40: 261:53
     let mut _12: std::string::String;    // in scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:261:55: 261:58
     let mut _13: rustc_errors::Level;    // in scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:261:60: 261:65
     let mut _14: std::option::Option<(std::string::String, std::vec::Vec<rustc_span::InnerSpan>)>; // in scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:261:67: 261:73
     scope 1 {
         debug level => _7;               // in scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:256:9: 256:14
     }
 
     bb0: {
         _6 = discriminant(((*_1).2: rustc_session::config::Lto)); // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:253:27: 253:35
-        switchInt(move _6) -> [1_isize: bb3, 3_isize: bb3, otherwise: bb2]; // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:253:27: 253:35
+        goto -> bb2;                     // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:253:27: 253:35
     }
 
     bb1 (cleanup): {
         resume;                          // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:243:1: 262:2
     }
 
     bb2: {
-        goto -> bb4;                     // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:253:5: 255:6
-    }
-
-    bb3: {
         _4 = const 0_u32;                // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:254:9: 254:19
-        goto -> bb4;                     // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:253:5: 255:6
+        goto -> bb3;                     // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:253:5: 255:6
     }
 
-    bb4: {
+    bb3: {
         StorageLive(_7);                 // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:256:9: 256:14
         _8 = discriminant(_3);           // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:257:9: 257:37
-        switchInt(move _8) -> [0_isize: bb6, 1_isize: bb7, 2_isize: bb8, 3_isize: bb8, otherwise: bb5]; // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:257:9: 257:37
+        switchInt(move _8) -> [0_isize: bb5, 1_isize: bb6, 2_isize: bb7, 3_isize: bb7, otherwise: bb4]; // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:257:9: 257:37
     }
 
-    bb5: {
+    bb4: {
         unreachable;                     // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:256:23: 256:28
     }
 
-    bb6: {
+    bb5: {
         discriminant(_7) = 2;            // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:257:41: 257:53
-        goto -> bb9;                     // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:256:17: 260:6
+        goto -> bb8;                     // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:256:17: 260:6
     }
 
-    bb7: {
+    bb6: {
         discriminant(_7) = 3;            // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:258:43: 258:57
-        goto -> bb9;                     // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:256:17: 260:6
+        goto -> bb8;                     // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:256:17: 260:6
     }
 
-    bb8: {
+    bb7: {
         discriminant(_7) = 4;            // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:259:72: 259:83
-        goto -> bb9;                     // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:256:17: 260:6
+        goto -> bb8;                     // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:256:17: 260:6
     }
 
-    bb9: {
+    bb8: {
         StorageLive(_9);                 // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:5: 261:74
         StorageLive(_10);                // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:5: 261:22
         _10 = &((*_1).20: rustc_codegen_ssa::back::write::SharedEmitter); // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:5: 261:22
         StorageLive(_11);                // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:40: 261:53
         _11 = _4;                        // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:40: 261:46
         StorageLive(_12);                // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:55: 261:58
         _12 = move _2;                   // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:55: 261:58
         StorageLive(_13);                // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:60: 261:65
         _13 = _7;                        // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:60: 261:65
         StorageLive(_14);                // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:67: 261:73
         _14 = move _5;                   // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:67: 261:73
-        _9 = SharedEmitter::inline_asm_error(move _10, move _11, move _12, move _13, move _14) -> [return: bb10, unwind: bb1]; // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:5: 261:74
+        _9 = SharedEmitter::inline_asm_error(move _10, move _11, move _12, move _13, move _14) -> [return: bb9, unwind: bb1]; // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:5: 261:74
                                          // mir::Constant
                                          // + span: compiler/rustc_codegen_llvm/src/back/write.rs:261:23: 261:39
                                          // + literal: Const { ty: for<'r> fn(&'r rustc_codegen_ssa::back::write::SharedEmitter, u32, std::string::String, rustc_errors::Level, std::option::Option<(std::string::String, std::vec::Vec<rustc_span::InnerSpan>)>) {rustc_codegen_ssa::back::write::SharedEmitter::inline_asm_error}, val: Value(Scalar(<ZST>)) }
     }
 
-    bb10: {
+    bb9: {
         StorageDead(_14);                // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:73: 261:74
         StorageDead(_13);                // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:73: 261:74
         StorageDead(_12);                // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:73: 261:74
         StorageDead(_11);                // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:73: 261:74
         StorageDead(_10);                // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:73: 261:74
         StorageDead(_9);                 // scope 1 at compiler/rustc_codegen_llvm/src/back/write.rs:261:74: 261:75
         _0 = const ();                   // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:249:3: 262:2
         StorageDead(_7);                 // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:262:1: 262:2
         return;                          // scope 0 at compiler/rustc_codegen_llvm/src/back/write.rs:262:2: 262:2
     }
