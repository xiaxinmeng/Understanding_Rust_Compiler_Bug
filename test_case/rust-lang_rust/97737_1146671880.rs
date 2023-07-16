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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/ty/print/pretty.rs at line 2190:
         // this is not *quite* right and changes the ordering of some output
         // anyways.
         let (new_value, map) = if self.tcx().sess.verbose() {
-            let regions: Vec<_> = value.bound_vars().into_iter().map(|var| {
-                let ty::BoundVariableKind::Region(var) = var else {
+            let regions: Vec<_> = value
+                .bound_vars()
+                .into_iter()
+                .map(|var| {
+                    let ty::BoundVariableKind::Region(var) = var else {
                     // This doesn't really matter because it doesn't get used,
                     // it's just an empty value
                     return ty::BrAnon(0);
Diff in /checkout/compiler/rustc_middle/src/ty/print/pretty.rs at line 2198:
-                match var {
-                match var {
-                    ty::BrAnon(_) | ty::BrEnv => {
-                        start_or_continue(&mut self, "for<", ", ");
-                        let name = next_name(&self);
-                        do_continue(&mut self, name);
-                        ty::BrNamed(CRATE_DEF_ID.to_def_id(), name)
+                    match var {
+                        ty::BrAnon(_) | ty::BrEnv => {
+                            start_or_continue(&mut self, "for<", ", ");
+                            let name = next_name(&self);
+                            do_continue(&mut self, name);
+                            ty::BrNamed(CRATE_DEF_ID.to_def_id(), name)
+                        }
+                        ty::BrNamed(def_id, kw::UnderscoreLifetime) => {
+                            start_or_continue(&mut self, "for<", ", ");
+                            let name = next_name(&self);
+                            do_continue(&mut self, name);
+                            ty::BrNamed(def_id, name)
+                        }
+                        ty::BrNamed(def_id, name) => {
+                            start_or_continue(&mut self, "for<", ", ");
+                            do_continue(&mut self, name);
+                            ty::BrNamed(def_id, name)
                     }
                     }
-                    ty::BrNamed(def_id, kw::UnderscoreLifetime) => {
-                        start_or_continue(&mut self, "for<", ", ");
-                        let name = next_name(&self);
-                        do_continue(&mut self, name);
-                        ty::BrNamed(def_id, name)
-                    }
-                    ty::BrNamed(def_id, name) => {
-                        start_or_continue(&mut self, "for<", ", ");
-                        do_continue(&mut self, name);
-                        ty::BrNamed(def_id, name)
-                }
-            }).collect();
+                })
+                .collect();
+                .collect();
             start_or_continue(&mut self, "", "> ");
 
             self.tcx.replace_late_bound_regions(value.clone(), |br| {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/structural_impls.rs" "/checkout/compiler/rustc_middle/src/ty/consts.rs" "/checkout/src/etc/test-float-parse/src/bin/u32-small.rs" "/checkout/compiler/rustc_middle/src/ty/error.rs" "/checkout/compiler/rustc_middle/src/ty/fast_reject.rs" "/checkout/compiler/rustc_middle/src/ty/adjustment.rs" "/checkout/compiler/rustc_middle/src/ty/print/pretty.rs" "/checkout/src/etc/test-float-parse/src/bin/u64-pow2.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
