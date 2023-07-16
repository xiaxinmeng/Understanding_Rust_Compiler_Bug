plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_cranelift/src/abi/comments.rs at line 82:
     }
     }
     let TyAndLayout { ty, layout } = place.layout();
-    let rustc_target::abi::LayoutS {
-        align,
-        abi: _,
-        variants: _,
-        fields: _,
-        fields: _,
-        niches: _,
-    } = layout.0.0;
+    let rustc_target::abi::LayoutS { size, align, abi: _, variants: _, fields: _, niches: _ } =
+        layout.0.0;
 
     let (kind, extra) = match *place.inner() {
         CPlaceInner::Var(place_local, var) => {
Diff in /checkout/compiler/rustc_codegen_cranelift/src/discriminant.rs at line 42:
         Variants::Multiple {
             tag: _,
             tag_field,
-            tag_encoding: TagEncoding::Niche { untagged_variant, ref niche_variants, niche_start, .. },
+            tag_encoding:
+                TagEncoding::Niche { untagged_variant, ref niche_variants, niche_start, .. },
             variants: _,
         } => {
             if variant_index != untagged_variant {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_cranelift/src/abi/comments.rs" "/checkout/compiler/rustc_codegen_cranelift/src/vtable.rs" "/checkout/compiler/rustc_codegen_cranelift/src/lib.rs" "/checkout/compiler/rustc_codegen_cranelift/src/abi/returning.rs" "/checkout/compiler/rustc_codegen_cranelift/src/abi/mod.rs" "/checkout/compiler/rustc_codegen_cranelift/src/abi/pass_mode.rs" "/checkout/compiler/rustc_codegen_cranelift/src/optimize/peephole.rs" "/checkout/compiler/rustc_codegen_cranelift/src/main_shim.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
