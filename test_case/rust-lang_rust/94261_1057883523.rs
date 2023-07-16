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
Diff in /checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs at line 1020:
         }
     };
 
-    debug_assert!(up_var_tys
-        .iter()
-        .all(|&t| t == cx.tcx.normalize_erasing_regions(ParamEnv::reveal_all(), t)));
+    debug_assert!(
+        up_var_tys
+            .iter()
+            .all(|&t| t == cx.tcx.normalize_erasing_regions(ParamEnv::reveal_all(), t))
 
 
     let capture_names = closure_saved_names_of_captured_variables(cx.tcx, def_id);
     let layout = cx.layout_of(closure_or_generator_ty);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/coverageinfo/mod.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm_util.rs" "/checkout/compiler/rustc_span/src/symbol/tests.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata/enums/cpp_like.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/mod.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/namespace.rs" "/checkout/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
