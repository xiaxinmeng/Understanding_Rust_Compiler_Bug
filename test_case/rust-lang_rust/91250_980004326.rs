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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_errors/src/emitter.rs at line 652:
     }
 
     fn maybe_anonymized(&self, line_num: usize) -> String {
-        if self.ui_testing {
-            ANONYMIZED_LINE_NUM.to_string()
-            line_num.to_string()
-        }
-        }
+        if self.ui_testing { ANONYMIZED_LINE_NUM.to_string() } else { line_num.to_string() }
 
 
     fn draw_line(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ty_utils/src/instance.rs" "/checkout/compiler/rustc_ty_utils/src/ty.rs" "/checkout/compiler/rustc_ty_utils/src/representability.rs" "/checkout/compiler/rustc_ty_utils/src/lib.rs" "/checkout/compiler/rustc_errors/src/emitter.rs" "/checkout/compiler/rustc_codegen_ssa/src/mono_item.rs" "/checkout/compiler/rustc_ty_utils/src/needs_drop.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
