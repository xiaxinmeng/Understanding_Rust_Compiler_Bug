plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
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
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs at line 10:
 use rustc_middle::infer::unify_key::ConstVariableOriginKind;
 use rustc_middle::ty::print::Print;
 use rustc_middle::ty::subst::{GenericArg, GenericArgKind};
-use rustc_middle::ty::{self, DefIdTree, InferConst, Ty, TyCtxt, GenericParamDefKind};
+use rustc_middle::ty::{self, DefIdTree, GenericParamDefKind, InferConst, Ty, TyCtxt};
 use rustc_span::source_map::DesugaringKind;
 use rustc_span::symbol::kw;
 use rustc_span::Span;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/at.rs" "/checkout/compiler/rustc_infer/src/infer/lattice.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/note.rs" "/checkout/compiler/rustc_infer/src/infer/glb.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/find_anon_type.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/different_lifetimes.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/named_anon_conflict.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
