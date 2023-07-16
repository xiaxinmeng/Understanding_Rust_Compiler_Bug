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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_llvm/src/debuginfo/utils.rs at line 74:
         ty::Dynamic(..) => Some(FatPtrKind::Dyn),
         ty::Adt(..) | ty::Tuple(..) if matches!(layout.variants, Variants::Single { .. }) => {
             let last_field_index = layout.fields.count() - 1;
-            debug_assert!((0..last_field_index)
-                .all(|field_index| { !layout.field(cx, field_index).is_unsized() }));
+            debug_assert!(
+                (0..last_field_index)
+                    .all(|field_index| { !layout.field(cx, field_index).is_unsized() })
 
 
             let unsized_field = layout.field(cx, last_field_index);
             debug_assert!(unsized_field.is_unsized());
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs" "/checkout/compiler/rustc_span/src/symbol/tests.rs" "/checkout/compiler/rustc_span/src/symbol.rs" "/checkout/compiler/rustc_codegen_llvm/src/asm.rs" "/checkout/compiler/rustc_span/src/analyze_source_file/tests.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/create_scope_map.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/utils.rs" "/checkout/compiler/rustc_span/src/source_map.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
