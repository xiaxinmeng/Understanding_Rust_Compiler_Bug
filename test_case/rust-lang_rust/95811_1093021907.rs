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
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs at line 16:
 use rustc_middle::ty::subst::GenericArgKind;
 use rustc_middle::ty::{self, Binder, ToPredicate, Ty};
 use rustc_span::symbol::{kw, sym};
-use rustc_span::{Span, MultiSpan};
+use rustc_span::{MultiSpan, Span};
 use rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt;
 use std::iter;
 use std::iter;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/mod.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_propagate.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_visualize.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs" "/checkout/compiler/rustc_typeck/src/coherence/builtin.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
