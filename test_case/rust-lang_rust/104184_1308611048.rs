
; git diff
diff --git a/src/bootstrap/config.rs b/src/bootstrap/config.rs
index 21dc11c4808..610536ff3d3 100644
--- a/src/bootstrap/config.rs
+++ b/src/bootstrap/config.rs
@@ -1595,23 +1595,7 @@ fn download_ci_rustc_commit(
     let compiler = format!("{top_level}/compiler/");
     let library = format!("{top_level}/library/");
 
-    // Look for a version to compare to based on the current commit.
-    // Only commits merged by bors will have CI artifacts.
-    let merge_base = output(
-        config
-            .git()
-            .arg("rev-list")
-            .arg(format!("--author={}", config.stage0_metadata.config.git_merge_commit_email))
-            .args(&["-n1", "--first-parent", "HEAD"]),
-    );
-    let commit = merge_base.trim_end();
-    if commit.is_empty() {
-        println!("error: could not find commit hash for downloading rustc");
-        println!("help: maybe your repository history is too shallow?");
-        println!("help: consider disabling `download-rustc`");
-        println!("help: or fetch enough history to include one upstream commit");
-        crate::detail_exit(1);
-    }
+    let commit = "a7cd917db3301da14641b8794327a9a4499838d8";
 
     // Warn if there were changes to the compiler or standard library since the ancestor commit.
     let has_changes = !t!(config
; x build
# ...
; /home/jnelson/rust-lang/rust2/build/x86_64-unknown-linux-gnu/ci-rustc-sysroot/bin/rustdoc --version
rustdoc 1.67.0-dev
; /home/jnelson/rust-lang/rust2/build/x86_64-unknown-linux-gnu/ci-rustc-sysroot/bin/rustc --version
rustc 1.67.0-nightly (a7cd917db 2022-11-09)
