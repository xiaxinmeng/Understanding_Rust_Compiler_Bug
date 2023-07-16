patch
diff --git a/rust/src/main.rs b/rust/src/main.rs
index 57b1f20..9476cc0 100644
--- a/rust/src/main.rs
+++ b/rust/src/main.rs
@@ -14,7 +14,7 @@ const NUM_GENERATIONS: i32 = 50;
 const NUM_SURVIVORS: usize = 100;

 type Point = (f64, f64);
-type Population = [[usize; NUM_POINTS]; MAX_POPULATION];
+type Population = [Vec<usize>; MAX_POPULATION];
 type World = [Point; NUM_POINTS];

 fn main() {
@@ -48,22 +48,22 @@ fn main() {
     println!("Elapsed: {:?}", elapsed);
 }

-fn new_random_agent(rng: &mut ThreadRng) -> [usize; NUM_POINTS]
+fn new_random_agent(rng: &mut ThreadRng) -> Vec<usize>
 {
     let mut agent: [usize; NUM_POINTS] = core::array::from_fn(|i| i);
     agent.shuffle(rng);
-    agent
+    agent.into()
 }

-fn new_agent_from_parents(parents: &[[usize; NUM_POINTS]], mut rng: &mut ThreadRng) -> [usize; NUM_POINTS] {
+fn new_agent_from_parents(parents: &[Vec<usize>], mut rng: &mut ThreadRng) -> Vec<usize> {
     let parent = parents.choose(&mut rng).unwrap();

-    let mut agent = *parent;
+    let mut agent = parent.clone();
     agent.swap(
         rng.gen_range(0..NUM_POINTS) as usize,
         rng.gen_range(0..NUM_POINTS) as usize,
     );
-    agent
+    agent.into()
 }

 fn distance(a: &Point, b: &Point) -> f64 {
