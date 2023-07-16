plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/queries.rs at line 190:
 
     /// Destructure a type-level constant ADT or array into its variant index and its field values.
     /// Panics if the destructuring fails, use `try_destructure_const` for fallible version.
-    pub fn destructure_const(
-        self,
-        const_: ty::Const<'tcx>,
-    ) -> mir::DestructuredConst<'tcx> {
+    pub fn destructure_const(self, const_: ty::Const<'tcx>) -> mir::DestructuredConst<'tcx> {
         self.try_destructure_const(const_).unwrap()
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/mir/interpret/mod.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/queries.rs" "/checkout/compiler/rustc_middle/src/tests.rs" "/checkout/compiler/rustc_target/src/abi/call/nvptx.rs" "/checkout/compiler/rustc_middle/src/thir.rs" "/checkout/compiler/rustc_target/src/abi/call/nvptx64.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/error.rs" "/checkout/compiler/rustc_middle/src/macros.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
