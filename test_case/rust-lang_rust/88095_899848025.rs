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
Diff in /checkout/compiler/rustc_middle/src/mir/query.rs at line 6:
     regions::{ConstraintSccIndex, OutlivesConstraint},
     Body, Promoted,
 };
-use crate::ty::{self, Ty, TyCtxt, RegionVid};
+use crate::ty::{self, RegionVid, Ty, TyCtxt};
 use rustc_data_structures::graph::scc::Sccs;
 use rustc_data_structures::stable_hasher::{HashStable, StableHasher};
 use rustc_data_structures::sync::Lrc;
Diff in /checkout/compiler/rustc_middle/src/ty/sty.rs at line 1071:
     where
         T: TypeFoldable<'tcx>,
     {
-        if self.0.has_escaping_bound_vars() {
-        } else {
-            Some(self.skip_binder())
-        }
-        }
+        if self.0.has_escaping_bound_vars() { None } else { Some(self.skip_binder()) }
 
 
     /// Splits the contents into two things that share the same binder
Diff in /checkout/compiler/rustc_middle/src/ty/sty.rs at line 1683:
     #[inline]
     #[inline]
     pub fn is_phantom_data(&self) -> bool {
-        if let Adt(def, _) = self.kind() {
-            def.is_phantom_data()
-            false
-        }
-        }
+        if let Adt(def, _) = self.kind() { def.is_phantom_data() } else { false }
 
     #[inline]
     #[inline]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/mir/interpret/pointer.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/allocation.rs" "/checkout/compiler/rustc_middle/src/mir/traversal.rs" "/checkout/compiler/rustc_middle/src/mir/query.rs" "/checkout/compiler/rustc_middle/src/thir.rs" "/checkout/compiler/rustc_middle/src/lib.rs" "/checkout/compiler/rustc_middle/src/hir/mod.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
