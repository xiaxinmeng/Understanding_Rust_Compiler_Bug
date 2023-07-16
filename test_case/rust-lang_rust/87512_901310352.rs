plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_llvm/src/type_of.rs at line 268:
         };
         debug!("--> mapped {:#?} to llty={:?}", self, llty);
 
-        cx.type_lowering.borrow_mut().insert(
-            (self.ty, variant_index),
-            TypeLowering { lltype: llty, field_remapping },
-        );
+        cx.type_lowering
+            .borrow_mut()
+            .insert((self.ty, variant_index), TypeLowering { lltype: llty, field_remapping });
 
         if let Some((llty, layout)) = defer {
             let (llfields, packed, new_field_remapping) = struct_llfields(cx, layout);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/llvm/diagnostic.rs" "/checkout/compiler/rustc_codegen_llvm/src/type_of.rs" "/checkout/compiler/rustc_codegen_llvm/src/context.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/mod.rs" "/checkout/compiler/rustc_codegen_llvm/src/callee.rs" "/checkout/compiler/rustc_codegen_llvm/src/allocator.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/utils.rs" "/checkout/compiler/rustc_codegen_llvm/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
