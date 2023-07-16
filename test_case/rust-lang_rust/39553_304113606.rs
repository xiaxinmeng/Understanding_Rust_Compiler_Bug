bash
$ git diff
diff --git a/src/memory.rs b/src/memory.rs
index 09529ab..0eb0aa2 100644
--- a/src/memory.rs
+++ b/src/memory.rs
@@ -558,7 +558,7 @@ impl Dictionary {
         prototype.get_key(freq_range, detectors)
     }

-    pub fn iter<'b>(&'b self) -> impl Iterator<Item=(&'b FragmentKey, &'b Box<Fragment>)> {
+    pub fn iter(&self) -> impl Iterator<Item=(&FragmentKey, &Box<Fragment>)> {
         self.map.iter()
     }

$ cargo +nightly build
   Compiling cortex v0.1.0 (file:///tmp/cortex)
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

thread 'rustc' panicked at 'assertion failed: match *region { ty::ReLateBound(..) => false, _ => true, }', /checkout/src/librustc/infer/higher_ranked/mod.rs:493
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: Could not compile `cortex`.

To learn more, run the command again with --verbose.

$ rustup run nightly rustc --version --verbose
rustc 1.19.0-nightly (5b13bff52 2017-05-23)
binary: rustc
commit-hash: 5b13bff5203c1bdc6ac6dc87f69b5359a9503078
commit-date: 2017-05-23
host: x86_64-unknown-linux-gnu
release: 1.19.0-nightly
LLVM version: 4.0
