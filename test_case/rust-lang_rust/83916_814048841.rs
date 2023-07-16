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
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 355:
                                 }),
                             }
                         }
-                        thir::InlineAsmOperand::Const { value, span } => mir::InlineAsmOperand::Const {
-                            value: box Constant {
-                                span,
-                                user_ty: None,
-                                literal: value.into(),
-                        },
-                        },
+                        thir::InlineAsmOperand::Const { value, span } => {
+                            mir::InlineAsmOperand::Const {
+                                value: box Constant { span, user_ty: None, literal: value.into() },
+                        }
+                        }
                         thir::InlineAsmOperand::SymFn { expr } => {
                             mir::InlineAsmOperand::SymFn { value: box this.as_constant(expr) }
                         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/build/matches/simplify.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/into.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/category.rs" "/checkout/compiler/rustc_mir_build/src/build/cfg.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/prove_predicate.rs" "/checkout/compiler/rustc_mir_build/src/build/misc.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/implied_outlives_bounds.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/stmt.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
