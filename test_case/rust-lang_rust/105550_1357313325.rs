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
Diff in /checkout/compiler/rustc_middle/src/values.rs at line 1:
+use crate::dep_graph::DepKind;
 use rustc_data_structures::fx::FxHashSet;
 use rustc_errors::{pluralize, struct_span_err, Applicability, MultiSpan};
 use rustc_hir as hir;
Diff in /checkout/compiler/rustc_middle/src/values.rs at line 4:
 use rustc_hir::def::DefKind;
 use rustc_middle::ty::Representability;
 use rustc_middle::ty::{self, DefIdTree, Ty, TyCtxt};
 use rustc_query_system::query::QueryInfo;
 use rustc_query_system::Value;
 use rustc_span::def_id::LocalDefId;
 use rustc_span::def_id::LocalDefId;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/list.rs" "/checkout/compiler/rustc_middle/src/ty/opaque_types.rs" "/checkout/compiler/rustc_mir_transform/src/const_debuginfo.rs" "/checkout/compiler/rustc_middle/src/traits/specialization_graph.rs" "/checkout/compiler/rustc_middle/src/ty/visit.rs" "/checkout/compiler/rustc_middle/src/traits/query.rs" "/checkout/compiler/rustc_middle/src/ty/_match.rs" "/checkout/compiler/rustc_middle/src/values.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
