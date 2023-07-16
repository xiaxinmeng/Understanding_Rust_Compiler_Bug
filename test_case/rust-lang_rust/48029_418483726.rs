`diff
diff --git a/src/librustc/session/mod.rs b/src/librustc/session/mod.rs
index 778c388c7d..2c05824e68 100644
--- a/src/librustc/session/mod.rs
+++ b/src/librustc/session/mod.rs
@@ -880,9 +880,11 @@ impl Session {
     /// compilation
     pub fn codegen_units(&self) -> usize {
         if let Some(n) = self.opts.cli_forced_codegen_units {
+                       eprintln!("path 1 was taken: {}", n);
             return n;
         }
         if let Some(n) = self.target.target.options.default_codegen_units {
+                       eprintln!("path 2 was taken {}", n);
             return n as usize;
         }
@@ -936,6 +938,7 @@ impl Session {
         // As a result 16 was chosen here! Mostly because it was a power of 2
         // and most benchmarks agreed it was roughly a local optimum. Not very
         // scientific.
+               eprintln!("path 3 was taken (16), default");
         16
     }
