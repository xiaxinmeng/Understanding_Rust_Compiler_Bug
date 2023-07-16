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
Diff in /checkout/src/librustdoc/clean/mod.rs at line 17:
 use rustc_hir::def_id::{CrateNum, DefId, CRATE_DEF_INDEX, LOCAL_CRATE};
 use rustc_index::vec::{Idx, IndexVec};
 use rustc_infer::infer::region_constraints::{Constraint, RegionConstraintData};
-use rustc_middle::{bug, span_bug};
 use rustc_middle::middle::resolve_lifetime as rl;
 use rustc_middle::ty::fold::TypeFolder;
 use rustc_middle::ty::subst::{InternalSubsts, Subst};
Diff in /checkout/src/librustdoc/clean/mod.rs at line 24:
 use rustc_middle::ty::{self, AdtKind, Lift, Ty, TyCtxt};
+use rustc_middle::{bug, span_bug};
 use rustc_mir::const_eval::{is_const_fn, is_unstable_const_fn};
 use rustc_span::hygiene::{AstPass, MacroKind};
 use rustc_span::symbol::{kw, sym, Ident, Symbol};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/cfg.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/visit_lib.rs" "/checkout/src/librustdoc/doctree.rs" "/checkout/src/etc/test-float-parse/rand-f64.rs" "/checkout/src/librustdoc/clean/cfg/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
