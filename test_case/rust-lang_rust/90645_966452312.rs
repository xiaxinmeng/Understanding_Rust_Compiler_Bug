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
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs at line 14:
 use rustc_middle::ty::{self, Binder, Ty};
 use rustc_span::symbol::{kw, sym};
-use std::iter;
-use std::iter;
 use rustc_middle::ty::subst::GenericArgKind;
+use std::iter;
 
 impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
     pub(in super::super) fn suggest_semicolon_at_end(
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs at line 285:
             {
                 if let ty::Adt(_def, subst) = found.kind() {
                     if subst.len() != 0 {
-                        if let GenericArgKind::Type(ty) = subst[0].unpack()
-                        {
+                        if let GenericArgKind::Type(ty) = subst[0].unpack() {
                             let peeled = ty.peel_refs().to_string();
                             if peeled == "String" {
                                 let ref_cnt = ty.to_string().len() - peeled.len();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/mod.rs" "/checkout/compiler/rustc_typeck/src/check/fallback.rs" "/checkout/compiler/rustc_typeck/src/check/dropck.rs" "/checkout/compiler/rustc_typeck/src/check/_match.rs" "/checkout/compiler/rustc_typeck/src/check/check.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
