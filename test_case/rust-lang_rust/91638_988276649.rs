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
Diff in /checkout/compiler/rustc_mir_transform/src/generator.rs at line 672:
     local_conflicts: BitMatrix<Local, Local>,
 
 
-impl<'mir, 'tcx> rustc_mir_dataflow::ResultsVisitor<'mir, 'tcx> for StorageConflictVisitor<'mir, 'tcx, '_> {
+impl<'mir, 'tcx> rustc_mir_dataflow::ResultsVisitor<'mir, 'tcx>
+    for StorageConflictVisitor<'mir, 'tcx, '_>
+{
     type FlowState = BitSet<Local>;
     fn visit_statement_before_primary_effect(
     fn visit_statement_before_primary_effect(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_transform/src/separate_const_switch.rs" "/checkout/compiler/rustc_builtin_macros/src/llvm_asm.rs" "/checkout/compiler/rustc_mir_transform/src/instcombine.rs" "/checkout/compiler/rustc_mir_transform/src/generator.rs" "/checkout/compiler/rustc_builtin_macros/src/util.rs" "/checkout/compiler/rustc_mir_transform/src/inline.rs" "/checkout/compiler/rustc_builtin_macros/src/derive.rs" "/checkout/compiler/rustc_mir_transform/src/remove_zsts.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
