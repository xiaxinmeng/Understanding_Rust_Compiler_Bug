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
Diff in /checkout/compiler/rustc_mir/src/transform/mod.rs at line 5:
 use rustc_hir as hir;
 use rustc_hir::def_id::{DefId, LocalDefId};
 use rustc_hir::intravisit::{self, NestedVisitorMap, Visitor};
+use rustc_hir::lang_items::LangItem;
 use rustc_index::vec::IndexVec;
 use rustc_middle::mir::visit::Visitor as _;
 use rustc_middle::mir::{traversal, Body, ConstQualifs, MirPhase, Promoted};
Diff in /checkout/compiler/rustc_mir/src/transform/mod.rs at line 12:
 use rustc_middle::ty::query::Providers;
 use rustc_middle::ty::{self, Ty, TyCtxt, TypeFoldable};
 use rustc_span::{Span, Symbol};
-use rustc_hir::lang_items::LangItem;
 use std::borrow::Cow;
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/simplify.rs" "/checkout/compiler/rustc_mir/src/transform/deduplicate_blocks.rs" "/checkout/compiler/rustc_mir/src/transform/mod.rs" "/checkout/compiler/rustc_mir/src/transform/multiple_return_terminators.rs" "/checkout/compiler/rustc_mir/src/transform/separate_const_switch.rs" "/checkout/compiler/rustc_mir/src/transform/check_packed_ref.rs" "/checkout/compiler/rustc_mir/src/transform/lower_intrinsics.rs" "/checkout/compiler/rustc_mir/src/transform/abort_unwinding_calls.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
