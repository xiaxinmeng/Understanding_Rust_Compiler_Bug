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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_resolve/src/late/lifetimes.rs at line 3282:
                                 ))
                                 .emit();
                         }
-                        hir::LifetimeName::Param(_)
-                        | hir::LifetimeName::Implicit(_) => {
+                        hir::LifetimeName::Param(_) | hir::LifetimeName::Implicit(_) => {
                             self.resolve_lifetime_ref(lt);
                         }
                         hir::LifetimeName::ImplicitObjectLifetimeDefault => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast_lowering/src/item.rs" "/checkout/compiler/rustc_driver/src/args.rs" "/checkout/compiler/rustc_traits/src/type_op.rs" "/checkout/compiler/rustc_driver/src/pretty.rs" "/checkout/compiler/rustc_traits/src/lib.rs" "/checkout/compiler/rustc_driver/src/lib.rs" "/checkout/compiler/rustc_traits/src/implied_outlives_bounds.rs" "/checkout/compiler/rustc_resolve/src/late/lifetimes.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
