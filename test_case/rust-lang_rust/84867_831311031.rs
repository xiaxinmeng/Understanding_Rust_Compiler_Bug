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
Diff in /checkout/src/librustdoc/passes/collect_trait_impls.rs at line 3:
 use crate::core::DocContext;
 use crate::fold::DocFolder;
 
-use rustc_data_structures::fx::{FxHashSet};
+use rustc_data_structures::fx::FxHashSet;
 use rustc_hir::def_id::{DefId, LOCAL_CRATE};
 use rustc_middle::ty::DefIdTree;
 use rustc_span::symbol::sym;
Diff in /checkout/src/librustdoc/html/render/context.rs at line 6:
 use std::sync::mpsc::{channel, Receiver};
 
 use rustc_data_structures::fx::{FxHashMap, FxHashSet};
-use rustc_hir::def_id::{LOCAL_CRATE};
+use rustc_hir::def_id::LOCAL_CRATE;
 use rustc_middle::ty::TyCtxt;
 use rustc_span::edition::Edition;
 use rustc_span::edition::Edition;
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1938:
             .any(|i| i.inner_impl().trait_.def_id_full(c) == c.deref_mut_trait_did);
         let inner_impl = target.def_id_full(c);
         debug!("target.def_id_full: {:?}", inner_impl);
-        let inner_impl = inner_impl
-            .or_else(|| {
-                target.primitive_type().and_then(|prim| c.primitive_locations.get(&prim).cloned())
-            });
+        let inner_impl = inner_impl.or_else(|| {
+            target.primitive_type().and_then(|prim| c.primitive_locations.get(&prim).cloned())
+        });
         debug!("inner_impl before impls[self.unwrap()] match: {:?}", inner_impl);
-        let inner_impl = inner_impl
-            .and_then(|did| c.impls.get(&did));
+        let inner_impl = inner_impl.and_then(|did| c.impls.get(&did));
         if let Some(impls) = inner_impl {
             debug!("found inner_impl: {:?}", impls);
             let mut used_links = FxHashSet::default();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/collect_trait_impls.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/core.rs" "/checkout/src/librustdoc/theme.rs" "/checkout/src/librustdoc/doctest/tests.rs" "/checkout/src/librustdoc/visit_ast.rs" "/checkout/src/librustdoc/markdown.rs" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
