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
Diff in /checkout/compiler/rustc_typeck/src/astconv/mod.rs at line 2125:
                             Res::Def(_, def_id) => self.tcx().opt_item_name(def_id),
                             _ => None,
-                        Some((match name {
-                        Some((match name {
-                            Some(ty) => format!("{desc} `{ty}`"),
-                            None => desc.to_string(),
-                        }, segment.ident.span))
+                            match name {
+                            match name {
+                                Some(ty) => format!("{desc} `{ty}`"),
+                                None => desc.to_string(),
+                            },
+                            segment.ident.span,
                     }
                 })
             })
             })
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/query/method_autoderef.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/dropck_outlives.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/normalize.rs" "/checkout/compiler/rustc_typeck/src/astconv/errors.rs" "/checkout/compiler/rustc_typeck/src/variance/constraints.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/implied_outlives_bounds.rs" "/checkout/compiler/rustc_typeck/src/astconv/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
