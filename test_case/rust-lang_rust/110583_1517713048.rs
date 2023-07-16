plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:4d706a8d448f000965ed9e883febfb36661202fe)
Complete job name: PR - mingw-check-tidy
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180119 sha256=9fa76c45f3193f307e02ea67d1a48cfe7ef2b854a074b766938a88e046bc7887
  Stored in directory: /tmp/pip-ephem-wheel-cache-ooceyzfs/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 2cb97331e580
Successfully tagged rust-ci:latest
Built container sha256:2cb97331e5804ad9df8470dcb1f59974912d99eb203a7c40d897c4735bf82d2b
Uploading finished image to https://ci-caches.rust-lang.org/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7
upload failed: - to s3://rust-lang-ci-sccache2/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
Diff in /checkout/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs at line 961:
         }
     }
 
+    fn suggest_make_local_mut(
+        &self,
+        err: &mut DiagnosticBuilder<'_, ErrorGuaranteed>,
+        local: Local,
+        name: Symbol,
+    ) {
+        let local_decl = &self.body.local_decls[local];
 
-fn suggest_make_local_mut(&self, err: &mut DiagnosticBuilder<'_, ErrorGuaranteed>, local: Local, name: Symbol) {
-    let local_decl = &self.body.local_decls[local];
+        let (pointer_sigil, pointer_desc) =
+            if local_decl.ty.is_ref() { ("&", "reference") } else { ("*const", "pointer") };
 
-    let (pointer_sigil, pointer_desc) =
-        if local_decl.ty.is_ref() { ("&", "reference") } else { ("*const", "pointer") };
+        let (is_trait_sig, local_trait) = self.is_error_in_trait(local);
+        if is_trait_sig && local_trait.is_none() {
+        }
 
 
-    let (is_trait_sig, local_trait) = self.is_error_in_trait(local);
-    if is_trait_sig && local_trait.is_none() {
-    }
+        let decl_span = match local_trait {
+        let decl_span = match local_trait {
+            Some(span) => span,
+            None => local_decl.source_info.span,
 
-    let decl_span = match local_trait {
-    let decl_span = match local_trait {
-        Some(span) => span,
-        None => local_decl.source_info.span
-    };
+        let label = match *local_decl.local_info() {
+            LocalInfo::User(mir::BindingForm::ImplicitSelf(_)) => {
+                let suggestion = suggest_ampmut_self(self.infcx.tcx, decl_span);
+                Some((true, decl_span, suggestion))
 
 
-    let label = match *local_decl.local_info() {
-        LocalInfo::User(mir::BindingForm::ImplicitSelf(_)) => {
-            let suggestion =
-                suggest_ampmut_self(self.infcx.tcx, decl_span);
-            Some((true, decl_span, suggestion))
-
-
-        LocalInfo::User(mir::BindingForm::Var(mir::VarBindingForm {
-            binding_mode: ty::BindingMode::BindByValue(_),
-            opt_ty_info,
-        })) => {
-        })) => {
-            // check if the RHS is from desugaring
-            let opt_assignment_rhs_span =
-                self.body.find_assignments(local).first().map(|&location| {
-                    if let Some(mir::Statement {
-                        source_info: _,
-                            mir::StatementKind::Assign(box (
-                                _,
-                                _,
-                                mir::Rvalue::Use(mir::Operand::Copy(place)),
-                            )),
-                    }) = self.body[location.block]
-                        .statements
-                        .get(location.statement_index)
-                    {
-                        self.body.local_decls[place.local].source_info.span
-                    } else {
-                        self.body.source_info(location).span
+            LocalInfo::User(mir::BindingForm::Var(mir::VarBindingForm {
+                binding_mode: ty::BindingMode::BindByValue(_),
+                opt_ty_info,
+            })) => {
+            })) => {
+                // check if the RHS is from desugaring
+                let opt_assignment_rhs_span =
+                    self.body.find_assignments(local).first().map(|&location| {
+                        if let Some(mir::Statement {
+                            source_info: _,
+                                mir::StatementKind::Assign(box (
+                                    _,
+                                    _,
+                                    mir::Rvalue::Use(mir::Operand::Copy(place)),
+                                )),
+                        }) = self.body[location.block].statements.get(location.statement_index)
+                        {
+                            self.body.local_decls[place.local].source_info.span
+                        } else {
+                            self.body.source_info(location).span
+                    });
+                    });
+                match opt_assignment_rhs_span.and_then(|s| s.desugaring_kind()) {
+                    // on for loops, RHS points to the iterator part
+                    Some(DesugaringKind::ForLoop) => {
+                        self.suggest_similar_mut_method_for_for_loop(err);
+                        err.span_label(
+                            opt_assignment_rhs_span.unwrap(),
+                            format!("this iterator yields `{pointer_sigil}` {pointer_desc}s",),
+                        None
                     }
-                });
-                });
-            match opt_assignment_rhs_span.and_then(|s| s.desugaring_kind()) {
-                // on for loops, RHS points to the iterator part
-                Some(DesugaringKind::ForLoop) => {
-                    self.suggest_similar_mut_method_for_for_loop(err);
-                    err.span_label(opt_assignment_rhs_span.unwrap(), format!(
-                        "this iterator yields `{pointer_sigil}` {pointer_desc}s",
-                    None
-                }
-                // don't create labels for compiler-generated spans
-                Some(_) => None,
-                Some(_) => None,
-                None => {
-                    let label = if name != kw::SelfLower {
-                        suggest_ampmut(
-                            self.infcx.tcx,
-                            local_decl.ty,
-                            decl_span,
-                            opt_assignment_rhs_span,
-                            opt_ty_info,
-                    } else {
-                        match local_decl.local_info() {
-                        match local_decl.local_info() {
-                            LocalInfo::User(mir::BindingForm::Var(
-                                mir::VarBindingForm {
-                                    opt_ty_info: None, ..
-                            )) => {
-                            )) => {
-                                let sugg = suggest_ampmut_self(
-                                    self.infcx.tcx,
-                                    decl_span,
-                                );
-                                (true, decl_span, sugg)
-                            }
-                            // explicit self (eg `self: &'a Self`)
-                            _ => suggest_ampmut(
+                    // don't create labels for compiler-generated spans
+                    Some(_) => None,
+                    None => {
+                        let label = if name != kw::SelfLower {
+                            suggest_ampmut(
                                 self.infcx.tcx,
                                 local_decl.ty,
                                 decl_span,
Diff in /checkout/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs at line 1050:
                                 opt_assignment_rhs_span,
                                 opt_ty_info,
-                        }
-                    };
-                    Some(label)
+                            )
+                            )
+                        } else {
+                            match local_decl.local_info() {
+                                LocalInfo::User(mir::BindingForm::Var(mir::VarBindingForm {
+                                    opt_ty_info: None,
+                                })) => {
+                                })) => {
+                                    let sugg = suggest_ampmut_self(self.infcx.tcx, decl_span);
+                                    (true, decl_span, sugg)
+                                }
+                                // explicit self (eg `self: &'a Self`)
+                                _ => suggest_ampmut(
+                                    self.infcx.tcx,
+                                    local_decl.ty,
+                                    decl_span,
+                                    opt_assignment_rhs_span,
+                                    opt_ty_info,
+                            }
+                        };
+                        Some(label)
+                    }
+                    }
                 }
             }
-        }
 
-        LocalInfo::User(mir::BindingForm::Var(mir::VarBindingForm {
-            binding_mode: ty::BindingMode::BindByReference(_),
-        })) => {
-        })) => {
-            let pattern_span: Span = local_decl.source_info.span;
-            suggest_ref_mut(self.infcx.tcx, pattern_span)
-                .map(|span| (true, span, "mut ".to_owned()))
-        }
+            LocalInfo::User(mir::BindingForm::Var(mir::VarBindingForm {
+                binding_mode: ty::BindingMode::BindByReference(_),
+            })) => {
+            })) => {
+                let pattern_span: Span = local_decl.source_info.span;
+                suggest_ref_mut(self.infcx.tcx, pattern_span)
+                    .map(|span| (true, span, "mut ".to_owned()))
 
-        _ => unreachable!(),
-    };
+            _ => unreachable!(),
+            _ => unreachable!(),
+        };
 
-    match label {
-        Some((true, err_help_span, suggested_code)) => {
-            err.span_suggestion_verbose(
-                err_help_span,
-                &format!(
-                    "consider changing this to be a mutable {pointer_desc}"
-                suggested_code,
-                Applicability::MachineApplicable,
-            );
-        }
-        }
-        Some((false, err_label_span, message)) => {
-            struct BindingFinder {
-                span: Span,
-                hir_id: Option<hir::HirId>,
+        match label {
+            Some((true, err_help_span, suggested_code)) => {
+                err.span_suggestion_verbose(
+                    err_help_span,
+                    &format!("consider changing this to be a mutable {pointer_desc}"),
+                    Applicability::MachineApplicable,
+                );
             }
             }
+            Some((false, err_label_span, message)) => {
+                struct BindingFinder {
+                    span: Span,
+                    hir_id: Option<hir::HirId>,
 
 
-            impl<'tcx> Visitor<'tcx> for BindingFinder {
-                fn visit_stmt(&mut self, s: &'tcx hir::Stmt<'tcx>) {
-                    if let hir::StmtKind::Local(local) = s.kind {
-                        if local.pat.span == self.span {
-                            self.hir_id = Some(local.hir_id);
+                impl<'tcx> Visitor<'tcx> for BindingFinder {
+                    fn visit_stmt(&mut self, s: &'tcx hir::Stmt<'tcx>) {
+                        if let hir::StmtKind::Local(local) = s.kind {
+                            if local.pat.span == self.span {
+                                self.hir_id = Some(local.hir_id);
                         }
+                        hir::intravisit::walk_stmt(self, s);
                     }
-                    hir::intravisit::walk_stmt(self, s);
-                    hir::intravisit::walk_stmt(self, s);
                 }
-            }
-            let hir_map = self.infcx.tcx.hir();
-            let def_id = self.body.source.def_id();
-            let hir_id = hir_map.local_def_id_to_hir_id(def_id.expect_local());
-            let node = hir_map.find(hir_id);
-            let hir_id = if let Some(hir::Node::Item(item)) = node
+                let hir_map = self.infcx.tcx.hir();
+                let def_id = self.body.source.def_id();
+                let hir_id = hir_map.local_def_id_to_hir_id(def_id.expect_local());
+                let node = hir_map.find(hir_id);
+                let hir_id = if let Some(hir::Node::Item(item)) = node
                 && let hir::ItemKind::Fn(.., body_id) = item.kind
             {
                 let body = hir_map.body(body_id);
Diff in /checkout/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs at line 1113:
                 None
             };
             };
-            if let Some(hir_id) = hir_id
+                if let Some(hir_id) = hir_id
                 && let Some(hir::Node::Local(local)) = hir_map.find(hir_id)
             {
                 let (changing, span, sugg) = match local.ty {
Diff in /checkout/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs at line 1138:
                 );
             }
+            }
+            None => {}
+            None => {}
         }
-        None => {}
     }
 }
-}
 
 pub fn mut_borrow_of_mutable_ref(local_decl: &LocalDecl<'_>, local_name: Option<Symbol>) -> bool {
     debug!("local_info: {:?}, ty.kind(): {:?}", local_decl.local_info, local_decl.ty.kind());
Diff in /checkout/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs at line 1171:
 }
 
 
-fn suggest_ampmut_self<'tcx>(
-    tcx: TyCtxt<'tcx>,
-    span: Span,
-) -> String {
+fn suggest_ampmut_self<'tcx>(tcx: TyCtxt<'tcx>, span: Span) -> String {
     match tcx.sess.source_map().span_to_snippet(span) {
         Ok(snippet) => {
             let lt_pos = snippet.find('\'');
Build completed unsuccessfully in 0:00:23
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_borrowck/src/diagnostics/explain_borrow.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/outlives_suggestion.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/var_name.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/region_name.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/bound_region_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/find_use.rs" "/checkout/compiler/rustc_borrowck/src/constraints/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
