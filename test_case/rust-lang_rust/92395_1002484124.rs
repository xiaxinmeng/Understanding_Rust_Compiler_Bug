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
Diff in /checkout/src/librustdoc/clean/inline.rs at line 5:
 
 use rustc_ast as ast;
 use rustc_data_structures::fx::FxHashSet;
+use rustc_data_structures::thin_vec::ThinVec;
 use rustc_hir as hir;
 use rustc_hir::def::{DefKind, Res};
 use rustc_hir::def_id::DefId;
Diff in /checkout/src/librustdoc/clean/inline.rs at line 14:
 use rustc_middle::ty::{self, TyCtxt};
 use rustc_span::hygiene::MacroKind;
 use rustc_span::symbol::{kw, sym, Symbol};
-use rustc_data_structures::thin_vec::ThinVec;
 use crate::clean::{
 use crate::clean::{
     self, clean_fn_decl_from_did_and_sig, clean_ty_generics, utils, Attributes, AttributesExt,
Diff in /checkout/src/librustdoc/clean/utils.rs at line 10:
 use rustc_ast as ast;
 use rustc_ast::tokenstream::TokenTree;
+use rustc_data_structures::thin_vec::ThinVec;
 use rustc_hir as hir;
 use rustc_hir as hir;
 use rustc_hir::def::{DefKind, Res};
 use rustc_hir::def_id::{DefId, LOCAL_CRATE};
Diff in /checkout/src/librustdoc/clean/utils.rs at line 19:
 use rustc_span::symbol::{kw, sym, Symbol};
 use std::mem;
-use rustc_data_structures::thin_vec::ThinVec;
 
 #[cfg(test)]
 #[cfg(test)]
 mod tests;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/layout.rs" "/checkout/src/librustdoc/doctest/tests.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/clean/utils/tests.rs" "/checkout/src/librustdoc/clean/simplify.rs" "/checkout/src/librustdoc/clean/types.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/html/markdown/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
