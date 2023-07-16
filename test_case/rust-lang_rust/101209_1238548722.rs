diff
diff --git a/.cargo/config.toml b/.cargo/config.toml
index af119a9..0bd940a 100644
--- a/.cargo/config.toml
+++ b/.cargo/config.toml
@@ -4,4 +4,8 @@ target = "x86_64-unknown-none"
 
 # Unused as we have to provide them via an environmental variable. Build doesn't work if we
 # only specifcy them here.
-# rustflags = []
+rustflags = [
+    "-C", "code-model=kernel",
+    "-C", "link-arg=-Tlink.ld",
+    "-C", "relocation-model=static",
+]
