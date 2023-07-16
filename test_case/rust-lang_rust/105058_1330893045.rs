plain
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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/tools/tidy/src/no_merge.rs at line 4:
     let Ok(remote) = get_rust_lang_rust_remote() else { return; };
     let Ok(merge_commits) = find_merge_commits(&remote) else { return; };
-    let mut bad_merge_commits = merge_commits
-        .lines()
-        .lines()
-        .filter(|commit| {
-            !(
-                // Bors is the ruler of merge commits.
-                commit.starts_with("Auto merge of")
+    let mut bad_merge_commits = merge_commits.lines().filter(|commit| {
+        !(
+            // Bors is the ruler of merge commits.
+            commit.starts_with("Auto merge of")
                 || commit.starts_with("Rollup merge of")
                 // Our fellow subtrees are fine as well.
                 || commit.contains("Merge from rustc")
Diff in /checkout/src/tools/tidy/src/no_merge.rs at line 16:
                 || commit.contains("merge rustc history")
-                || commit.contains("rust-analyzer"))
-                || commit.contains("clippy")
-        });
+                || commit.contains("rust-analyzer")
+        ) || commit.contains("clippy")
 
 
     if let Some(merge) = bad_merge_commits.next() {
         tidy_error!(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/tidy/src/no_merge.rs" "/checkout/src/tools/tidy/src/debug_artifacts.rs" "/checkout/src/tools/tidy/src/main.rs" "/checkout/src/tools/tidy/src/unit_tests.rs" "/checkout/src/tools/tidy/src/style.rs" "/checkout/src/tools/tidy/src/errors.rs" "/checkout/src/tools/tidy/src/extdeps.rs" "/checkout/src/bootstrap/tool.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
