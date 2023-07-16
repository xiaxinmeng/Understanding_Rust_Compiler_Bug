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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_resolve/src/late/lifetimes.rs at line 8:
 
 use rustc_ast::walk_list;
 use rustc_data_structures::fx::{FxHashSet, FxIndexMap, FxIndexSet};
-use rustc_errors::{struct_span_err};
+use rustc_errors::struct_span_err;
 use rustc_hir as hir;
 use rustc_hir::def::{DefKind, Res};
 use rustc_hir::def_id::LocalDefId;
Diff in /checkout/compiler/rustc_resolve/src/late/lifetimes.rs at line 23:
 use rustc_span::symbol::{sym, Ident};
 use rustc_span::Span;
-
 
 trait RegionExt {
 trait RegionExt {
     fn early(hir_map: Map<'_>, param: &GenericParam<'_>) -> (LocalDefId, Region);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_resolve/src/late/lifetimes.rs" "/checkout/compiler/rustc_resolve/src/late.rs" "/checkout/compiler/rustc_resolve/src/lib.rs" "/checkout/compiler/rustc_resolve/src/diagnostics/tests.rs" "/checkout/compiler/rustc_smir/src/lib.rs" "/checkout/compiler/rustc_const_eval/src/errors.rs" "/checkout/compiler/rustc_smir/src/very_unstable.rs" "/checkout/compiler/rustc_resolve/src/late/diagnostics.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
