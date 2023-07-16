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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/project.rs at line 19:
 use crate::infer::type_variable::{TypeVariableOrigin, TypeVariableOriginKind};
 use crate::infer::{InferCtxt, InferOk, LateBoundRegionConversionTime};
 use crate::traits::error_reporting::InferCtxtExt;
-use rustc_data_structures::stack::ensure_sufficient_stack;
 use rustc_data_structures::sso::SsoHashSet;
+use rustc_data_structures::stack::ensure_sufficient_stack;
 use rustc_errors::ErrorReported;
 use rustc_hir::def_id::DefId;
 use rustc_hir::lang_items::LangItem;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/util.rs" "/checkout/compiler/rustc_trait_selection/src/traits/specialize/specialization_graph.rs" "/checkout/compiler/rustc_trait_selection/src/traits/object_safety.rs" "/checkout/compiler/rustc_trait_selection/src/traits/structural_match.rs" "/checkout/compiler/rustc_trait_selection/src/traits/project.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/normalize.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/auto_trait.rs"` failed.
Build completed unsuccessfully in 0:00:16
Build completed unsuccessfully in 0:00:16
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
