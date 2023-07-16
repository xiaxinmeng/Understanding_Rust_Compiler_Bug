plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v1.0.0
    Checking adler v1.0.2
    Checking rustc-demangle v0.1.21
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v1.0.0
    Checking adler v1.0.2
    Checking rustc-demangle v0.1.21
---
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/library/test)
    Finished release [optimized] target(s) in 19.07s
Checking stage0 std test/bench/example targets (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
    Checking ppv-lite86 v0.2.8
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking test v0.0.0 (/checkout/library/test)
    Checking rand_core v0.5.1
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v1.0.0
   Compiling adler v1.0.2
   Compiling rustc-demangle v0.1.21
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/iter/traits/double_ended.rs at line 134:
     /// [`Err(k)`]: Err
     #[inline]
     #[unstable(feature = "iter_advance_by", reason = "recently added", issue = "77404")]
-    fn advance_back_by(&mut self, n: usize) -> Result<(), usize> where Self::Item: ~const Destruct {
+    fn advance_back_by(&mut self, n: usize) -> Result<(), usize>
+    where
+        Self::Item: ~const Destruct,
+    {
         // FIXME(const_trait_impl): revert back to for loop
         let mut i = 0;
         while i < n {
Diff in /checkout/library/core/src/iter/traits/double_ended.rs at line 187:
     #[inline]
     #[inline]
     #[stable(feature = "iter_nth_back", since = "1.37.0")]
-    fn nth_back(&mut self, n: usize) -> Option<Self::Item> where Self::Item: ~const Destruct {
+    fn nth_back(&mut self, n: usize) -> Option<Self::Item>
+    where
+        Self::Item: ~const Destruct,
+    {
         self.advance_back_by(n).ok()?;
         self.next_back()
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/iter/traits/double_ended.rs" "/checkout/library/core/src/iter/adapters/inspect.rs" "/checkout/library/core/src/iter/traits/iterator.rs" "/checkout/library/core/src/iter/adapters/enumerate.rs" "/checkout/library/core/src/iter/traits/marker.rs" "/checkout/library/core/src/iter/adapters/mod.rs" "/checkout/library/core/src/iter/traits/mod.rs" "/checkout/library/core/src/iter/adapters/step_by.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
