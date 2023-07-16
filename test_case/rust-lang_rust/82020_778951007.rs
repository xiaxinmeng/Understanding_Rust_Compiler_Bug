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
Diff in /checkout/src/librustdoc/clean/auto_trait.rs at line 283:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/auto_trait.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 
         let orig_bounds: FxHashSet<_> =
             self.cx.tcx.param_env(param_env_def_id).caller_bounds().iter().collect();
-        let clean_where_predicates = param_env
-            .caller_bounds()
-            .iter()
-            .filter(|p| {
-                !orig_bounds.contains(p)
-                    || match p.kind().skip_binder() {
-                        ty::PredicateKind::Trait(pred, _) => pred.def_id() == sized_trait,
-                    }
-            });
-            });
+        let clean_where_predicates = param_env.caller_bounds().iter().filter(|p| {
+            !orig_bounds.contains(p)
+                || match p.kind().skip_binder() {
+                    ty::PredicateKind::Trait(pred, _) => pred.def_id() == sized_trait,
+                }
+        });
 
         let mut generic_params =
         let mut generic_params =
             (tcx.generics_of(param_env_def_id), tcx.explicit_predicates_of(param_env_def_id))
Build completed unsuccessfully in 0:00:23
