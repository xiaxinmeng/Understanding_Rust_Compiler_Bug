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
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs at line 526:
             //   |                             ^^^^^^^ cannot infer type
             //   |
             //   = note: cannot resolve `<_ as std::ops::Try>::Ok == _`
-            if span.contains(*call_span) {
-                *call_span
-                span
-            }
-            }
+            if span.contains(*call_span) { *call_span } else { span }
             span
         };
         };
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/util.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/placeholder_error.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/different_lifetimes.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/find_anon_type.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/mod.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/named_anon_conflict.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
