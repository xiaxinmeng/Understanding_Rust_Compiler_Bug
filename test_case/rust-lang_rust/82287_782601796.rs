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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_typeck/src/check/expr.rs at line 1356:
         displayable_field_names.sort();
         let mut truncated_fields_error = String::new();
-        let remaining_fields_names =
-        let remaining_fields_names =
-            match &displayable_field_names[..] {
-                [field1] => format!("`{}`", field1),
-                [field1, field2] => format!("`{}` and `{}`", field1, field2),
-                [field1, field2, field3] => format!("`{}`, `{}` and `{}`", field1, field2, field3),
-                _ => {
-                    truncated_fields_error = format!(" and {} other field{}", len - 3, pluralize!(len - 3));
-                    displayable_field_names
-                        .iter()
-                        .take(3)
-                        .map(|n| format!("`{}`", n))
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/expr.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-                        .collect::<Vec<_>>()
-                        .join(", ")
-            };
-            };
+        let remaining_fields_names = match &displayable_field_names[..] {
+            [field1] => format!("`{}`", field1),
+            [field1, field2] => format!("`{}` and `{}`", field1, field2),
+            [field1, field2, field3] => format!("`{}`, `{}` and `{}`", field1, field2, field3),
+                truncated_fields_error =
+                truncated_fields_error =
+                    format!(" and {} other field{}", len - 3, pluralize!(len - 3));
+                displayable_field_names
+                    .iter()
+                    .take(3)
+                    .map(|n| format!("`{}`", n))
+                    .collect::<Vec<_>>()
+                    .join(", ")
+        };
 
         struct_span_err!(
         struct_span_err!(
             self.tcx.sess,
Build completed unsuccessfully in 0:00:17
