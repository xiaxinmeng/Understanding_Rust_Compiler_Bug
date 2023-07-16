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
Diff in /checkout/src/bootstrap/channel.rs at line 56:
         let short_ver_hash = output(
             Command::new("git").current_dir(dir).arg("rev-parse").arg("--short=9").arg("HEAD"),
-        GitInfo::Present(
-            Info {
-            Info {
-                commit_date: ver_date.trim().to_string(),
-                sha: ver_hash.trim().to_string(),
-                short_sha: short_ver_hash.trim().to_string(),
-        )
+        GitInfo::Present(Info {
+        GitInfo::Present(Info {
+            commit_date: ver_date.trim().to_string(),
+            sha: ver_hash.trim().to_string(),
+            short_sha: short_ver_hash.trim().to_string(),
     }
 
 
     fn info(&self) -> Option<&Info> {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/prelude/mod.rs" "/checkout/src/bootstrap/build.rs" "/checkout/library/std/src/prelude/v1.rs" "/checkout/src/bootstrap/channel.rs" "/checkout/library/std/src/process/tests.rs" "/checkout/library/std/src/sync/barrier/tests.rs" "/checkout/library/std/src/sync/condvar/tests.rs" "/checkout/src/bootstrap/compile.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
