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
Diff in /checkout/library/std/src/path.rs at line 1031:
     // the middle of one
     if left.prefix.is_none() && right.prefix.is_none() && left.front == right.front {
         // possible future improvement: a [u8]::first_mismatch simd implementation
-        let first_difference =
-            match left.path.iter().zip(right.path).position(|(&a, &b)| a != b) {
-                None if left.path.len() == right.path.len() => return cmp::Ordering::Equal,
-                None => left.path.len().min(right.path.len()),
-                Some(diff) => diff,
-            };
+        let first_difference = match left.path.iter().zip(right.path).position(|(&a, &b)| a != b) {
+            None if left.path.len() == right.path.len() => return cmp::Ordering::Equal,
+            None => left.path.len().min(right.path.len()),
+            Some(diff) => diff,
 
 
         if let Some(previous_sep) =
             left.path[..first_difference].iter().rposition(|&b| left.is_sep_byte(b))
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/ffi/os_str.rs" "/checkout/library/std/src/ffi/mod.rs" "/checkout/library/std/src/ffi/os_str/tests.rs" "/checkout/library/std/src/ffi/c_str/tests.rs" "/checkout/library/std/src/path.rs" "/checkout/library/std/src/sys_common/wtf8.rs" "/checkout/library/std/src/sys_common/fs.rs" "/checkout/library/std/src/ffi/c_str.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
