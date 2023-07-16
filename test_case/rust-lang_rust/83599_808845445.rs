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
Diff in /checkout/compiler/rustc_hir/src/pat_util.rs at line 1:
 use crate::def::{CtorOf, DefKind, Res};
 use crate::def_id::DefId;
 use crate::hir::{self, HirId, PatKind};
+use rustc_data_structures::stable_set::FxHashSet;
 use rustc_span::symbol::Ident;
 use rustc_span::Span;
-use rustc_data_structures::stable_set::FxHashSet;
 
 use std::iter::{Enumerate, ExactSizeIterator};
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/resolve.rs" "/checkout/compiler/rustc_hir/src/weak_lang_items.rs" "/checkout/compiler/rustc_infer/src/infer/at.rs" "/checkout/compiler/rustc_hir/src/def.rs" "/checkout/compiler/rustc_infer/src/infer/equate.rs" "/checkout/compiler/rustc_hir/src/definitions.rs" "/checkout/compiler/rustc_hir/src/pat_util.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/env.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
