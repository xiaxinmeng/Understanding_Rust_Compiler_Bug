plain
Successfully built a782d64d8228
Successfully tagged rust-ci:latest
Built container sha256:a782d64d8228658fe3cfe42961037061b219e0c52153bbaaa3bf8a07910daa24
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
* 393 features
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/bootstrap/config.rs at line 887:
         // We still support running outside the repository if we find we aren't in a git directory.
         cmd.arg("rev-parse").arg("--show-toplevel");
         // Discard stderr because we expect this to fail when building from a tarball.
-        let output = cmd.stderr(std::process::Stdio::null()).output().ok().and_then(|output| {
-            if output.status.success() {
-                Some(output)
-                None
-            }
-        });
+        let output = cmd
+        let output = cmd
+            .stderr(std::process::Stdio::null())
+            .output()
+            .ok()
+            .and_then(|output| if output.status.success() { Some(output) } else { None });
         if let Some(output) = output {
             let git_root = String::from_utf8(output.stdout).unwrap();
             // We need to canonicalize this path to make sure it uses backslashes instead of forward slashes.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/bin/rustc.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/tarball.rs" "/checkout/src/bootstrap/test.rs" "/checkout/src/bootstrap/dylib_util.rs" "/checkout/src/bootstrap/channel.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
