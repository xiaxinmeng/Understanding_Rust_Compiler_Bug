plain
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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_resolve/src/diagnostics.rs at line 6:
 use rustc_ast_pretty::pprust;
 use rustc_data_structures::fx::FxHashSet;
 use rustc_errors::struct_span_err;
-use rustc_errors::{
-    Applicability, Diagnostic, DiagnosticBuilder, ErrorGuaranteed, MultiSpan,
-};
+use rustc_errors::{Applicability, Diagnostic, DiagnosticBuilder, ErrorGuaranteed, MultiSpan};
 use rustc_feature::BUILTIN_ATTRIBUTES;
 use rustc_hir::def::Namespace::{self, *};
 use rustc_hir::def::{self, CtorKind, CtorOf, DefKind, NonMacroAttrKind, PerNS};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_dataflow/src/framework/lattice.rs" "/checkout/compiler/rustc_incremental/src/lib.rs" "/checkout/compiler/rustc_resolve/src/def_collector.rs" "/checkout/compiler/rustc_incremental/src/assert_module_sources.rs" "/checkout/compiler/rustc_resolve/src/effective_visibilities.rs" "/checkout/compiler/rustc_incremental/src/persist/save.rs" "/checkout/compiler/rustc_resolve/src/diagnostics.rs" "/checkout/compiler/rustc_symbol_mangling/src/v0.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
