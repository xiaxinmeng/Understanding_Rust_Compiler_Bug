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
Diff in /checkout/compiler/rustc_codegen_llvm/src/attributes.rs at line 335:
             .iter()
             .find(|a| a.has_name(rustc_span::sym::target_feature))
             .map_or_else(|| cx.tcx.def_span(instance.def_id()), |a| a.span);
-        let msg =
-            format!("the target features {} must all be either enabled or disabled together", f.join(", "));
+        let msg = format!(
+            "the target features {} must all be either enabled or disabled together",
+            f.join(", ")
+        );
         let mut err = cx.tcx.sess.struct_span_err(span, &msg);
         err.help("add the missing features in a `target_feature` attribute");
         err.emit();
Diff in /checkout/compiler/rustc_codegen_llvm/src/llvm_util.rs at line 449:
     if let Some(f) =
         check_tied_features(sess, &feats.iter().map(|f| (strip(f), !f.starts_with("-"))).collect())
     {
-        sess.err(&format!("Target features {} must all be enabled or disabled together", f.join(", ")));
+        sess.err(&format!(
+            "Target features {} must all be enabled or disabled together",
+            f.join(", ")
     }
     }
     features.extend(feats.iter().flat_map(|&f| filter(f)));
     features
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/lints.rs" "/checkout/compiler/rustc_codegen_llvm/src/asm.rs" "/checkout/compiler/rustc_log/src/lib.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm_util.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs" "/checkout/compiler/rustc_codegen_llvm/src/declare.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/mod.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/as_place.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
