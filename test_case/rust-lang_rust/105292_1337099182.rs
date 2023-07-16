plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs at line 156:
                 ControlFlow::BREAK
             } else if let ty::ConstKind::Expr(e) = c.kind() {
                 if let ty::Expr::Cast(_cast, ct, ty) = e {
-                    if ty == self.ct.ty()
-                            .infcx
-                            .infcx
-                            .can_eq(self.param_env, ct, self.ct)
-                            .is_ok()
+                    if ty == self.ct.ty() && self.infcx.can_eq(self.param_env, ct, self.ct).is_ok()
                         ControlFlow::BREAK
                     } else {
                     } else {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/engine.rs" "/checkout/compiler/rustc_trait_selection/src/traits/util.rs" "/checkout/compiler/rustc_trait_selection/src/traits/specialize/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs" "/checkout/compiler/rustc_trait_selection/src/traits/specialize/specialization_graph.rs" "/checkout/compiler/rustc_trait_selection/src/traits/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/wf.rs" "/checkout/compiler/rustc_errors/src/json/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
