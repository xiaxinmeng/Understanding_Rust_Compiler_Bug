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
Diff in /checkout/src/bootstrap/lib.rs at line 484:
         // NOTE: The check for the empty directory is here because when running
         // x.py the first time, the submodule won't be checked out. Check it out
         // now so we can build it.
-        if !channel::GitInfo::new(false, relative_path).is_git() && !dir_is_empty(&self.config.src.join(relative_path)) {
+        if !channel::GitInfo::new(false, relative_path).is_git()
+            && !dir_is_empty(&self.config.src.join(relative_path))
             return;
         }
 
Diff in /checkout/src/bootstrap/lib.rs at line 506:
Diff in /checkout/src/bootstrap/lib.rs at line 506:
                 .arg(relative_path)
                 .current_dir(&self.config.src),
-        let hash =
-        let hash =
-            recorded.split(' ').nth(2).unwrap_or_else(|| panic!("unexpected output `{}`", recorded));
+        let hash = recorded
+            .split(' ')
+            .nth(2)
+            .unwrap_or_else(|| panic!("unexpected output `{}`", recorded));
         // update_submodule
         // update_submodule
         if let Some(existing_hash) = checked_out {
Diff in /checkout/src/bootstrap/lib.rs at line 561:
             "src/tools/cargo",
             "src/tools/rls",
             "src/tools/miri",
-            "library/backtrace"
+            "library/backtrace",
         ];
-        let output = output(Command::new("git")
-            .args(&["config", "--file"]).arg(&self.config.src.join(".gitmodules")).args(&["--get-regexp", "path"])
+        let output = output(
+            Command::new("git")
+                .args(&["config", "--file"])
+                .arg(&self.config.src.join(".gitmodules"))
+                .args(&["--get-regexp", "path"]),
         for line in output.lines() {
         for line in output.lines() {
             let submodule = Path::new(line.splitn(2, ' ').nth(1).unwrap());
Diff in /checkout/src/bootstrap/lib.rs at line 571:
             // avoid updating submodules twice
-            if !BOOTSTRAP_SUBMODULES.iter().any(|&p| Path::new(p) == submodule) && channel::GitInfo::new(false, submodule).is_git() {
+            if !BOOTSTRAP_SUBMODULES.iter().any(|&p| Path::new(p) == submodule)
+                && channel::GitInfo::new(false, submodule).is_git()
+            {
                 self.update_submodule(submodule);
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/term/src/win.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/util.rs" "/checkout/library/alloc/tests/slice.rs" "/checkout/library/alloc/src/raw_vec.rs" "/checkout/library/term/src/terminfo/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
