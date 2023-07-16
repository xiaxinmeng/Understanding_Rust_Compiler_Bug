diff
diff --git a/Cargo.toml b/Cargo.toml
index c27e5c469cf..f1430e2b1c9 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -89,6 +89,8 @@ rustfmt-nightly = { path = "src/tools/rustfmt" }
 # here
 rustc-workspace-hack = { path = 'src/tools/rustc-workspace-hack' }

+libc = {path="libc-0.2.79"}
+
 # See comments in `library/rustc-std-workspace-core/README.md` for what's going on
 # here
 rustc-std-workspace-core = { path = 'library/rustc-std-workspace-core' }
