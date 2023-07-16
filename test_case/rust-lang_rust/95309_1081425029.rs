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
Diff in /checkout/compiler/rustc_typeck/src/check/dropck.rs at line 4:
 use rustc_errors::{struct_span_err, ErrorGuaranteed};
 use rustc_middle::ty::error::TypeError;
 use rustc_middle::ty::relate::{Relate, RelateResult, TypeRelation};
-use rustc_middle::ty::subst::{SubstsRef};
+use rustc_middle::ty::subst::SubstsRef;
 use rustc_middle::ty::{self, Predicate, Ty, TyCtxt};
 use rustc_span::Span;
 use rustc_trait_selection::traits::query::dropck_outlives::AtExt;
Diff in /checkout/compiler/rustc_typeck/src/check/dropck.rs at line 11:
-use rustc_trait_selection::traits::{ObligationCause};
+use rustc_trait_selection::traits::ObligationCause;
 
 /// This function confirms that the `Drop` implementation identified by
 /// `drop_impl_did` is not any more specialized than the type it is
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/dropck.rs" "/checkout/compiler/rustc_typeck/src/variance/solve.rs" "/checkout/compiler/rustc_typeck/src/variance/test.rs" "/checkout/compiler/rustc_typeck/src/check/pat.rs" "/checkout/compiler/rustc_typeck/src/variance/xform.rs" "/checkout/compiler/rustc_typeck/src/check/callee.rs" "/checkout/compiler/rustc_typeck/src/check/mod.rs" "/checkout/compiler/rustc_typeck/src/variance/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
