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
Diff in /checkout/compiler/rustc_ast_passes/src/ast_validation.rs at line 1655:
                 walk_list!(self, visit_ty, ty);
             }
             AssocItemKind::Fn(box FnKind(_, ref sig, ref generics, ref body))
-                if self.in_const_trait_impl || ctxt == AssocCtxt::Trait || matches!(sig.header.constness, Const::Yes(_)) =>
+                if self.in_const_trait_impl
+                    || ctxt == AssocCtxt::Trait
+                    || matches!(sig.header.constness, Const::Yes(_)) =>
             {
                 self.visit_vis(&item.vis);
                 self.visit_ident(item.ident);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast_passes/src/feature_gate.rs" "/checkout/compiler/rustc_ast_passes/src/show_span.rs" "/checkout/compiler/rustc_ast_passes/src/ast_validation.rs" "/checkout/compiler/rustc_apfloat/tests/ppc.rs" "/checkout/compiler/rustc_apfloat/tests/ieee.rs" "/checkout/compiler/rustc_apfloat/src/ppc.rs" "/checkout/compiler/rustc_apfloat/src/ieee.rs" "/checkout/compiler/rustc_ast_passes/src/node_count.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
