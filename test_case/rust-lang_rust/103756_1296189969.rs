patch
diff --git a/rust/src/main.rs b/rust/src/main.rs
index 57b1f20..1f201b7 100644
--- a/rust/src/main.rs
+++ b/rust/src/main.rs
@@ -18,6 +18,10 @@ type Population = [[usize; NUM_POINTS]; MAX_POPULATION];
 type World = [Point; NUM_POINTS];

 fn main() {
+    std::thread::Builder::new().stack_size(100_000_00).spawn(run).unwrap().join().unwrap();
+}
+
+fn run() {
     // set up world
     let world: World = core::array::from_fn(|_| (fastrand::f64(), fastrand::f64()));
