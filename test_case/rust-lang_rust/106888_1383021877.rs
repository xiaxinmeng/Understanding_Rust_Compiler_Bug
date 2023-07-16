plain
Successfully built 2b4e14cba467
Successfully tagged rust-ci:latest
Built container sha256:2b4e14cba467646f4964836d6a32eaa8223acf5ae6192333ecb63574acc83743
Uploading finished image to https://ci-caches.rust-lang.org/docker/d33e4deadd55ac310bf6d14046967e2ae76df1d0cbcfb3cbdfd57a515110e5c43bea0fc68be40129a2ca62a8c3c0cddee210b82b000006fe1b3d251a4634baf3
upload failed: - to s3://rust-lang-ci-sccache2/docker/d33e4deadd55ac310bf6d14046967e2ae76df1d0cbcfb3cbdfd57a515110e5c43bea0fc68be40129a2ca62a8c3c0cddee210b82b000006fe1b3d251a4634baf3 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
Highest error code: `E0791`
* 394 features
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/tools/tidy/src/rustdoc_gui_tests.rs at line 3:
 use std::path::Path;
 
 pub fn check(path: &Path, bad: &mut bool) {
-    crate::walk::walk(&path.join("rustdoc-gui"), &mut |p| {
-        // If there is no extension, it's very likely a folder and we want to go into it.
-        p.extension().map(|e| e != "goml").unwrap_or(false)
-    }, &mut |entry, content| {
-        for line in content.lines() {
-            if !line.starts_with("// ") {
-                tidy_error!(
-                    bad,
-                    "{}: rustdoc-gui tests must start with a small description",
-                    entry.path().display(),
-                return;
-                return;
-            } else if line.starts_with("// ") {
-                let parts = line[2..].trim();
-                // We ignore tidy comments.
-                if parts.starts_with("// tidy-") {
+    crate::walk::walk(
+    crate::walk::walk(
+        &path.join("rustdoc-gui"),
+        &mut |p| {
+            // If there is no extension, it's very likely a folder and we want to go into it.
+            p.extension().map(|e| e != "goml").unwrap_or(false)
+        },
+        &mut |entry, content| {
+            for line in content.lines() {
+                if !line.starts_with("// ") {
+                    tidy_error!(
+                        bad,
+                        "{}: rustdoc-gui tests must start with a small description",
+                        entry.path().display(),
+                    return;
+                    return;
+                } else if line.starts_with("// ") {
+                    let parts = line[2..].trim();
+                    // We ignore tidy comments.
+                    if parts.starts_with("// tidy-") {
+                    }
+                    // All good!
+                    return;
                 }
---
+        },
+    );
 }
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/tidy/src/edition.rs" "/checkout/src/tools/tidy/src/error_codes.rs" "/checkout/src/tools/tidy/src/ui_tests.rs" "/checkout/src/tools/tidy/src/lib.rs" "/checkout/src/tools/remote-test-server/src/main.rs" "/checkout/src/tools/rust-demangler/src/lib.rs" "/checkout/src/tools/rust-demangler/src/main.rs" "/checkout/src/tools/tidy/src/rustdoc_gui_tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
