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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/fulfill.rs at line 407:
                     let pred =
                         ty::Binder::dummy(infcx.replace_bound_vars_with_placeholders(binder));
                     ProcessResult::Changed(mk_pending(vec![
-                        obligation.with(pred.to_predicate(self.selcx.tcx()))
+                        obligation.with(pred.to_predicate(self.selcx.tcx())),
                     ]))
                 }
                 ty::PredicateKind::TypeWellFormedFromEnv(..) => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/fulfill.rs" "/checkout/compiler/rustc_trait_selection/src/traits/relationships.rs" "/checkout/compiler/rustc_expand/src/module.rs" "/checkout/compiler/rustc_trait_selection/src/lib.rs" "/checkout/compiler/rustc_expand/src/mbe.rs" "/checkout/compiler/rustc_trait_selection/src/infer.rs" "/checkout/compiler/rustc/src/main.rs" "/checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
