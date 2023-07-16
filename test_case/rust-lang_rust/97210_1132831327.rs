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
Diff in /checkout/src/bootstrap/flags.rs at line 82:
     pub allow: Option<String>,
     pub deny: Option<String>,
     pub warn: Option<String>,
-    pub forbid: Option<String>
+    pub forbid: Option<String>,
 
 #[cfg_attr(test, derive(Clone))]
Diff in /checkout/src/bootstrap/check.rs at line 44:
         }
         }
         args.extend(strings(&["--", "--cap-lints", "warn"]));
         args.extend(ignored_lints.iter().map(|lint| format!("-Aclippy::{}", lint)));
-        args.extend(builder.config.clippy_adwf.iter().enumerate().filter(|(_, value)| value.is_some()).map(|(index, value)| format!("-{}{}", match index {
-            0 => "A",
-            1 => "D",
-            2 => "W",
-            3 => "F",
-            _ => unreachable!()
-        }, value.as_ref().unwrap())));
+        args.extend(
+            builder.config.clippy_adwf.iter().enumerate().filter(|(_, value)| value.is_some()).map(
+                |(index, value)| {
+                    format!(
+                        "-{}{}",
+                        match index {
+                            0 => "A",
+                            1 => "D",
+                            2 => "W",
+                            3 => "F",
+                            _ => unreachable!(),
+                        value.as_ref().unwrap()
+                    )
+                },
+            ),
+            ),
+        );
         dbg!(args.clone());
     } else {
     } else {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/bin/llvm-config-wrapper.rs" "/checkout/src/bootstrap/bin/main.rs" "/checkout/src/librustdoc/doctest.rs" "/checkout/src/bootstrap/check.rs" "/checkout/src/librustdoc/core.rs" "/checkout/src/bootstrap/cc_detect.rs" "/checkout/src/librustdoc/docfs.rs" "/checkout/src/librustdoc/visit_ast.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
