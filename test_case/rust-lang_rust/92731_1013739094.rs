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
Diff in /checkout/compiler/rustc_passes/src/intrinsicck.rs at line 411:
                             let msg = format!(
                                 "register class `{}` requires at least one of the following target features: {}",
                                 reg_class.name(),
-                                features.iter().map(|f| f.as_str()).intersperse(", ").collect::<String>(),
+                                    .iter()
+                                    .iter()
+                                    .map(|f| f.as_str())
+                                    .intersperse(", ")
+                                    .collect::<String>(),
                             );
                             self.tcx.sess.struct_span_err(*op_sp, &msg).emit();
                             // register isn't enabled, don't do more checks
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_passes/src/stability.rs" "/checkout/compiler/rustc_passes/src/check_attr.rs" "/checkout/compiler/rustc_passes/src/hir_stats.rs" "/checkout/compiler/rustc_passes/src/intrinsicck.rs" "/checkout/compiler/rustc_passes/src/upvars.rs" "/checkout/compiler/rustc_parse/src/parser/path.rs" "/checkout/compiler/rustc_passes/src/layout_test.rs" "/checkout/compiler/rustc_parse/src/parser/attr_wrapper.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
