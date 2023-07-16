diff
diff --git a/src/libstd/thread/local.rs b/src/libstd/thread/local.rs
index a170abb262..79293acc2c 100644
--- a/src/libstd/thread/local.rs
+++ b/src/libstd/thread/local.rs
@@ -177,6 +177,7 @@ macro_rules! __thread_local_inner {
                     $crate::thread::__StaticLocalKeyInner::new();
 
                 #[thread_local]
+                #[cfg_attr(target_os = "macos", link_section = "__DATA,__thread_data")]
                 #[cfg(all(target_thread_local, not(target_arch = "wasm32")))]
                 static __KEY: $crate::thread::__FastLocalKeyInner<$t> =
                     $crate::thread::__FastLocalKeyInner::new();
