rust
index dc7fa53aab..f2ae7e99df 100644
--- a/src/libstd/termination.rs
+++ b/src/libstd/termination.rs
@@ -8,7 +8,7 @@
 // option. This file may not be copied, modified, or distributed
 // except according to those terms.
 
-use fmt::Debug;
+use fmt::Display;
 #[cfg(target_arch = "wasm32")]
 mod exit {
     pub const SUCCESS: i32 = 0;
@@ -45,12 +45,12 @@ impl Termination for () {
 }
 
 #[unstable(feature = "termination_trait", issue = "43301")]
-impl<T: Termination, E: Debug> Termination for Result<T, E> {
+impl<T: Termination, E: Display> Termination for Result<T, E> {
     fn report(self) -> i32 {
         match self {
             Ok(val) => val.report(),
             Err(err) => {
-                eprintln!("Error: {:?}", err);
+                eprintln!("Error: {}", err);
                 exit::FAILURE
             }
         }
