 patch
diff --git a/mk/crates.mk b/mk/crates.mk
index c7abf27..f68fc2a 100644
--- a/mk/crates.mk
+++ b/mk/crates.mk
@@ -78,7 +78,6 @@ DEPS_panic_abort := libc alloc
 DEPS_panic_unwind := libc alloc unwind
 DEPS_unwind := libc

-RUSTFLAGS_compiler_builtins := -lstatic=compiler-rt

 # FIXME(stage0): change this to just `RUSTFLAGS_panic_abort := ...`
 RUSTFLAGS1_panic_abort := -C panic=abort
