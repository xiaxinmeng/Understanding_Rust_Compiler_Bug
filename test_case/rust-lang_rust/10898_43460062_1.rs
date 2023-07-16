 patch
diff --git a/src/libtest/lib.rs b/src/libtest/lib.rs
index 4681c02..9517961 100644
--- a/src/libtest/lib.rs
+++ b/src/libtest/lib.rs
@@ -156,6 +156,20 @@ impl TestFn {
     }
 }

+impl fmt::Show for TestFn {
+    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
+        f.write(match *self {
+            StaticTestFn(..) => "StaticTestFn(..)",
+            StaticBenchFn(..) => "StaticBenchFn(..)",
+            StaticMetricFn(..) => "StaticMetricFn(..)",
+            DynTestFn(..) => "DynTestFn(..)",
+            DynMetricFn(..) => "DynMetricFn(..)",
+            DynBenchFn(..) => "DynBenchFn(..)"
+        }.as_bytes()).ok().unwrap();
+        Ok(())
+    }
+}
+
 /// Manager of the benchmarking runs.
 ///
 /// This is feed into functions marked with `#[bench]` to allow for
@@ -170,13 +184,14 @@ pub struct Bencher {

 // The definition of a single test. A test runner will run a list of
 // these.
-#[deriving(Clone)]
+#[deriving(Clone, Show)]
 pub struct TestDesc {
     pub name: TestName,
     pub ignore: bool,
     pub should_fail: bool,
 }

+#[deriving(Show)]
 pub struct TestDescAndFn {
     pub desc: TestDesc,
     pub testfn: TestFn,
@@ -242,15 +257,9 @@ pub fn test_main(args: &[StrBuf], tests: Vec<TestDescAndFn> ) {
 pub fn test_main_static(args: &[StrBuf], tests: &[TestDescAndFn]) {
     let owned_tests = tests.iter().map(|t| {
         match t.testfn {
-            StaticTestFn(f) =>
-            TestDescAndFn { testfn: StaticTestFn(f), desc: t.desc.clone() },
-
-            StaticBenchFn(f) =>
-            TestDescAndFn { testfn: StaticBenchFn(f), desc: t.desc.clone() },
-
-            _ => {
-                fail!("non-static tests passed to test::test_main_static");
-            }
+            StaticTestFn(f) => TestDescAndFn { testfn: StaticTestFn(f), desc: t.desc.clone() },
+            StaticBenchFn(f) => TestDescAndFn { testfn: StaticBenchFn(f), desc: t.desc.clone() },
+            _ => fail!("non-static tests passed to test::test_main_static")
         }
     }).collect();
     test_main(args, owned_tests)
@@ -739,10 +748,9 @@ pub fn fmt_bench_samples(bs: &BenchSamples) -> StrBuf {
 }

 // A simple console test runner
-pub fn run_tests_console(opts: &TestOpts,
-                         tests: Vec<TestDescAndFn> ) -> io::IoResult<bool> {
-    fn callback<T: Writer>(event: &TestEvent,
-                           st: &mut ConsoleTestState<T>) -> io::IoResult<()> {
+pub fn run_tests_console(opts: &TestOpts, tests: Vec<TestDescAndFn> ) -> io::IoResult<bool> {
+
+    fn callback<T: Writer>(event: &TestEvent, st: &mut ConsoleTestState<T>) -> io::IoResult<()> {
         match (*event).clone() {
             TeFiltered(ref filtered_tests) => st.write_run_start(filtered_tests.len()),
             TeWait(ref test, padding) => st.write_test_start(test, padding),
@@ -778,6 +786,7 @@ pub fn run_tests_console(opts: &TestOpts,
             }
         }
     }
+
     let mut st = try!(ConsoleTestState::new(opts, None::<StdWriter>));
     fn len_if_padded(t: &TestDescAndFn) -> uint {
         match t.testfn.padding() {
@@ -933,9 +942,7 @@ fn get_concurrency() -> uint {
     }
 }

-pub fn filter_tests(
-    opts: &TestOpts,
-    tests: Vec<TestDescAndFn> ) -> Vec<TestDescAndFn> {
+pub fn filter_tests(opts: &TestOpts, tests: Vec<TestDescAndFn>) -> Vec<TestDescAndFn> {
     let mut filtered = tests;

     // Remove tests that don't match the test filter
@@ -972,6 +979,7 @@ pub fn filter_tests(
     match opts.test_shard {
         None => filtered,
         Some((a,b)) => {
+            println!("sharding over: {}", filtered);
             filtered.move_iter().enumerate()
             .filter(|&(i,_)| i % b == a)
             .map(|(_,t)| t)
