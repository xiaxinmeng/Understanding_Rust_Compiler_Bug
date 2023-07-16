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
Diff in /checkout/compiler/rustc_builtin_macros/src/test.rs at line 370:
             match attr.meta_item_list() {
                 // Handle #[ignore(bar = "foo")]
                 Some(_) => {
-                    sd.struct_span_err(i.span, r#"ignore with message should #[ignore = "message"]"#)
-                        .emit();
+                    sd.struct_span_err(
+                        i.span,
+                        r#"ignore with message should #[ignore = "message"]"#,
+                    )
+                    .emit();
-                },
+                }
+                }
                 // Handle #[ignore] and #[ignore = "message"]
                 None => attr.value_str(),
Diff in /checkout/compiler/rustc_builtin_macros/src/test.rs at line 380:
         }
         None => {
         None => {
-            sd.struct_span_err(i.span, r#"ignore a test should use #[ignore] or #[ignore = "message"]"#)
-                .emit();
+            sd.struct_span_err(
+                i.span,
+                r#"ignore a test should use #[ignore] or #[ignore = "message"]"#,
+            )
+            .emit();
-        },
+        }
     }
 }
 }
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_builtin_macros/src/format.rs" "/checkout/compiler/rustc_builtin_macros/src/compile_error.rs" "/checkout/compiler/rustc_builtin_macros/src/concat_bytes.rs" "/checkout/compiler/rustc_builtin_macros/src/trace_macros.rs" "/checkout/compiler/rustc_builtin_macros/src/format_foreign/shell/tests.rs" "/checkout/compiler/rustc_builtin_macros/src/test.rs" "/checkout/compiler/rustc_builtin_macros/src/lib.rs" "/checkout/compiler/rustc_builtin_macros/src/deriving/cmp/eq.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
