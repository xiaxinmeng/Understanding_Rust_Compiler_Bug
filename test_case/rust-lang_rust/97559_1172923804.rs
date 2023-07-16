plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
extracting /checkout/obj/build/cache/2022-05-20/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/ty/layout.rs at line 14:
 use rustc_span::symbol::Symbol;
 use rustc_span::{Span, DUMMY_SP};
 use rustc_target::abi::call::{
-    ArgAbi, ArgAttribute, ArgAttributes, ArgExtension, Conv, FnAbi, PassMode, /* Reg, RegKind, */
+    ArgAbi, ArgAttribute, ArgAttributes, ArgExtension, Conv, FnAbi,
+    PassMode, /* Reg, RegKind, */
 use rustc_target::abi::*;
 use rustc_target::abi::*;
 use rustc_target::spec::{abi::Abi as SpecAbi, HasTargetSpec, PanicStrategy, Target};
Diff in /checkout/compiler/rustc_middle/src/ty/layout.rs at line 3380:
 
                         if arg.layout.is_unsized() || size > max_by_val_size {
                             arg.make_indirect();
-                        // } else if self.has_all_float(&arg.layout) {
-                        //     // We don't want to aggregate floats as an aggregates of Integer
-                        //     // because this will hurt the generated assembly (#93490)
-                        //     //
-                        //     // As an optimization we want to pass homogeneous aggregate of floats
-                        //     // greater than pointer size as indirect
-                        //     if size > Pointer.size(self) {
-                        //         arg.make_indirect();
-                        // } else {
-                        //     // We want to pass small aggregates as immediates, but using
-                        //     // We want to pass small aggregates as immediates, but using
-                        //     // a LLVM aggregate type for this leads to bad optimizations,
-                        //     // so we pick an appropriately sized integer type instead.
-                        //     //
-                        //     // NOTE: This is sub-optimal because in the case of (f32, f32, u32, u32)
-                        //     // we could do ([f32; 2], u64) which is better but this is the best we
-                        //     // can do right now.
-                        //     arg.cast_to(Reg { kind: RegKind::Integer, size });
+                            // } else if self.has_all_float(&arg.layout) {
+                            //     // We don't want to aggregate floats as an aggregates of Integer
+                            //     // because this will hurt the generated assembly (#93490)
+                            //     //
+                            //     // As an optimization we want to pass homogeneous aggregate of floats
+                            //     // greater than pointer size as indirect
+                            //     if size > Pointer.size(self) {
+                            //         arg.make_indirect();
+                            // } else {
+                            //     // We want to pass small aggregates as immediates, but using
+                            //     // We want to pass small aggregates as immediates, but using
+                            //     // a LLVM aggregate type for this leads to bad optimizations,
+                            //     // so we pick an appropriately sized integer type instead.
+                            //     //
+                            //     // NOTE: This is sub-optimal because in the case of (f32, f32, u32, u32)
+                            //     // we could do ([f32; 2], u64) which is better but this is the best we
+                            //     // can do right now.
+                            //     arg.cast_to(Reg { kind: RegKind::Integer, size });
                     }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/closure.rs" "/checkout/compiler/rustc_middle/src/ty/diagnostics.rs" "/checkout/compiler/rustc_middle/src/ty/layout.rs" "/checkout/compiler/rustc_middle/src/ty/binding.rs" "/checkout/compiler/rustc_middle/src/ty/cast.rs" "/checkout/compiler/rustc_middle/src/ty/fold.rs" "/checkout/compiler/rustc_middle/src/ty/parameterized.rs" "/checkout/compiler/rustc_middle/src/ty/codec.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
