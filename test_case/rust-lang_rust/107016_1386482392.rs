diff
diff --git a/library/core/src/panicking.rs b/library/core/src/panicking.rs
index 48e90e6d794..3b7e6882107 100644
--- a/library/core/src/panicking.rs
+++ b/library/core/src/panicking.rs
@@ -164,7 +164,6 @@ fn panic_bounds_check(index: usize, len: usize) -> ! {
 /// This function is called directly by the codegen backend, and must not have
 /// any extra arguments (including those synthesized by track_caller).
 #[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
-#[cfg_attr(feature = "panic_immediate_abort", inline)]
 #[cfg_attr(bootstrap, lang = "panic_no_unwind")] // needed by codegen for panic in nounwind function
 #[cfg_attr(not(bootstrap), lang = "panic_cannot_unwind")] // needed by codegen for panic in nounwind function
 #[rustc_nounwind]
