
diff --git a/vendor/crossbeam-utils/no_atomic.rs b/rustc-1.62.1-src-new/vendor/crossbeam-utils/no_atomic.rs
index 390019e..f7a5fe4 100644
--- a/vendor/crossbeam-utils/no_atomic.rs
+++ b/vendor/crossbeam-utils/no_atomic.rs
@@ -43,6 +43,7 @@ const NO_ATOMIC_64: &[&str] = &[
     "powerpc-unknown-linux-gnu",
     "powerpc-unknown-linux-gnuspe",
     "powerpc-unknown-linux-musl",
+    "powerpc-gentoo-linux-musl",
     "powerpc-unknown-netbsd",
     "powerpc-unknown-openbsd",
     "powerpc-wrs-vxworks",
