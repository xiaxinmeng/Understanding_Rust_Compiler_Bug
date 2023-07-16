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
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/linker.rs at line 1558:
     symbols
 }
 
-pub(crate) fn linked_symbols(tcx: TyCtxt<'_>, crate_type: CrateType) -> Vec<(String, SymbolExportKind)> {
+pub(crate) fn linked_symbols(
+    tcx: TyCtxt<'_>,
+    crate_type: CrateType,
+) -> Vec<(String, SymbolExportKind)> {
     match crate_type {
         CrateType::Executable | CrateType::Cdylib => (),
         CrateType::Staticlib | CrateType::ProcMacro | CrateType::Rlib | CrateType::Dylib => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/back/rpath/tests.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/linker.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/metadata.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/write.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/symbol_export.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/analyze.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/archive.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/block.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
