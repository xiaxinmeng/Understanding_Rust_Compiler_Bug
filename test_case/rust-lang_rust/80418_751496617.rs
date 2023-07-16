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
Diff in /checkout/compiler/rustc_mir/src/transform/check_consts/validation.rs at line 3:
 use rustc_errors::{struct_span_err, Applicability, Diagnostic, ErrorReported};
 use rustc_hir::def_id::DefId;
 use rustc_hir::{self as hir, HirId, LangItem};
+use rustc_index::bit_set::BitSet;
 use rustc_infer::infer::TyCtxtInferExt;
 use rustc_infer::traits::{ImplSource, Obligation, ObligationCause};
 use rustc_middle::mir::visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor};
Diff in /checkout/compiler/rustc_mir/src/transform/check_consts/validation.rs at line 16:
 use rustc_span::{sym, Span, Symbol};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/check_consts/validation.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 use rustc_trait_selection::traits::error_reporting::InferCtxtExt;
 use rustc_trait_selection::traits::{self, SelectionContext, TraitEngine};
-use rustc_index::bit_set::BitSet;
 use std::mem;
 use std::ops::Deref;
 use std::ops::Deref;
Diff in /checkout/compiler/rustc_mir/src/transform/check_consts/validation.rs at line 289:
 
     fn local_has_storage_dead(&mut self, local: Local) -> bool {
         let ccx = self.ccx;
-        self.local_has_storage_dead.get_or_insert_with(|| {
-            struct StorageDeads {
-                locals: BitSet<Local>,
-            }
-            impl Visitor<'tcx> for StorageDeads {
-                fn visit_statement(&mut self, stmt: &Statement<'tcx>, _: Location) {
-                    if let StatementKind::StorageDead(l) = stmt.kind {
-                        self.locals.insert(l);
+        self.local_has_storage_dead
+            .get_or_insert_with(|| {
+                struct StorageDeads {
+                    locals: BitSet<Local>,
+                }
+                impl Visitor<'tcx> for StorageDeads {
+                    fn visit_statement(&mut self, stmt: &Statement<'tcx>, _: Location) {
+                        if let StatementKind::StorageDead(l) = stmt.kind {
+                            self.locals.insert(l);
                     }
                 }
-            }
-            let mut v = StorageDeads {
-            let mut v = StorageDeads {
-                locals: BitSet::new_empty(ccx.body.local_decls.len()),
-            };
-            v.visit_body(ccx.body);
-            v.locals
-        }).contains(local)
+                let mut v = StorageDeads { locals: BitSet::new_empty(ccx.body.local_decls.len()) };
+                v.visit_body(ccx.body);
+                v.locals
+            })
+            .contains(local)
 
 
     pub fn qualifs_in_return_place(&mut self) -> ConstQualifs {
Build completed unsuccessfully in 0:00:15
