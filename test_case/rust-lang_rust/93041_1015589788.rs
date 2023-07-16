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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/mir/mod.rs at line 1255:
     ResumedAfterPanic(GeneratorKind),
 
-#[derive(
-    Clone,
-    Debug,
-    Debug,
-    PartialEq,
-    TyEncodable,
-    TyDecodable,
-    Hash,
-    HashStable,
-    TypeFoldable
-)]
+#[derive(Clone, Debug, PartialEq, TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable)]
 pub enum InlineAsmOperand<'tcx> {
     In {
         reg: InlineAsmRegOrRegClass,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/mir/interpret/error.rs" "/checkout/compiler/rustc_middle/src/mir/mod.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/mod.rs" "/checkout/compiler/rustc_middle/src/mir/type_foldable.rs" "/checkout/compiler/rustc_middle/src/mir/predecessors.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/allocation.rs" "/checkout/compiler/rustc_middle/src/mir/query.rs" "/checkout/compiler/rustc_middle/src/mir/generic_graph.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
