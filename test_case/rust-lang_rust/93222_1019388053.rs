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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_interface/src/passes.rs at line 571:
         return false;
     }
     let check = |output_path: &PathBuf| {
-        if output_path.canonicalize().ok() == input_path {
-            Some(())
-            None
-        }
-        }
+        if output_path.canonicalize().ok() == input_path { Some(()) } else { None }
     };
     check_output(output_paths, check).is_some()
Diff in /checkout/compiler/rustc_typeck/src/check/compare_method.rs at line 688:
         }
     }
 
 
-    if let Some(reported) = err_occurred {
-        Err(reported)
-        Ok(())
-    }
-    }
+    if let Some(reported) = err_occurred { Err(reported) } else { Ok(()) }
 
 
 fn compare_number_of_method_arguments<'tcx>(
Diff in /checkout/compiler/rustc_typeck/src/check/compare_method.rs at line 920:
             error_found = Some(reported);
     }
     }
-    if let Some(reported) = error_found {
-        Err(reported)
-        Ok(())
-    }
-    }
+    if let Some(reported) = error_found { Err(reported) } else { Ok(()) }
 
 
 fn compare_const_param_types<'tcx>(
Diff in /checkout/compiler/rustc_session/src/output.rs at line 165:
         CrateType::Executable => {
             let suffix = &sess.target.exe_suffix;
             let out_filename = outputs.path(OutputType::Exe);
-            if suffix.is_empty() {
-            } else {
-            } else {
-                out_filename.with_extension(&suffix[1..])
-            }
+            if suffix.is_empty() { out_filename } else { out_filename.with_extension(&suffix[1..]) }
     }
 }
Diff in /checkout/compiler/rustc_session/src/output.rs at line 184:
 /// interaction with Rust code through static library is the only
 /// interaction with Rust code through static library is the only
 /// option for now
 pub fn default_output_for_target(sess: &Session) -> CrateType {
-    if !sess.target.executables {
-        CrateType::Staticlib
-        CrateType::Executable
-    }
-    }
+    if !sess.target.executables { CrateType::Staticlib } else { CrateType::Executable }
 
 
 /// Checks if target supports crate_type as output
Diff in /checkout/compiler/rustc_trait_selection/src/traits/fulfill.rs at line 390:
                     let pred =
                         ty::Binder::dummy(infcx.replace_bound_vars_with_placeholders(binder));
                     ProcessResult::Changed(mk_pending(vec![
-                        obligation.with(pred.to_predicate(self.selcx.tcx()))
+                        obligation.with(pred.to_predicate(self.selcx.tcx())),
                     ]))
                 }
                 ty::PredicateKind::TypeWellFormedFromEnv(..) => {
Diff in /checkout/compiler/rustc_typeck/src/check/wfcheck.rs at line 1416:
     check_where_clauses(fcx, span, def_id, Some((sig.output(), hir_decl.output.span())));
 
 
-const HELP_FOR_SELF_TYPE: &str =
-    "consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, \
+const HELP_FOR_SELF_TYPE: &str = "consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, \
      `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one \
      of the previous types except `Self`)";
Diff in /checkout/compiler/rustc_session/src/session.rs at line 801:
             Path::new(&rustlib_path),
             Path::new(&rustlib_path),
             Path::new("bin"),
-        if self_contained {
-        if self_contained {
-            vec![p.clone(), p.join("self-contained")]
-            vec![p]
-        }
-        }
+        if self_contained { vec![p.clone(), p.join("self-contained")] } else { vec![p] }
 
     pub fn init_incr_comp_session(
     pub fn init_incr_comp_session(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_interface/src/passes.rs" "/checkout/compiler/rustc_interface/src/queries.rs" "/checkout/compiler/rustc_privacy/src/lib.rs" "/checkout/compiler/rustc_ast/src/ptr.rs" "/checkout/compiler/rustc_hir_pretty/src/lib.rs" "/checkout/compiler/rustc_ast/src/attr/mod.rs" "/checkout/compiler/rustc_ast/src/ast_like.rs" "/checkout/compiler/rustc_interface/src/callbacks.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
