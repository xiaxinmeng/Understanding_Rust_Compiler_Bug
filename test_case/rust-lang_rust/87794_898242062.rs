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
Diff in /checkout/compiler/rustc_target/src/abi/mod.rs at line 989:
             return None;
 
-        let (start, valid_range) =
-        let (start, valid_range) =
-            if *v.start() <= max_value - *v.end() && *v.start() >= count {
-                //moves back until zero is reached
-                let start = v.start() - count;
-                (start, start..=*v.end())
-            } else {
-                let end = v.end().wrapping_add(count) & max_value;
-                let start = v.end().wrapping_add(1) & max_value;
-                (start, *v.start()..=end)
-            };
+        let (start, valid_range) = if *v.start() <= max_value - *v.end() && *v.start() >= count {
+            //moves back until zero is reached
+            let start = v.start() - count;
+            (start, start..=*v.end())
+        } else {
+            let end = v.end().wrapping_add(count) & max_value;
+            let start = v.end().wrapping_add(1) & max_value;
+            (start, *v.start()..=end)
+        };
         Some((start, Scalar { value, valid_range }))
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_serialize/src/json/tests.rs" "/checkout/compiler/rustc_expand/src/mbe/macro_check.rs" "/checkout/compiler/rustc_target/src/lib.rs" "/checkout/compiler/rustc_feature/src/active.rs" "/checkout/compiler/rustc_feature/src/lib.rs" "/checkout/compiler/rustc_feature/src/removed.rs" "/checkout/compiler/rustc_target/src/abi/mod.rs" "/checkout/compiler/rustc_expand/src/mbe/macro_rules.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
