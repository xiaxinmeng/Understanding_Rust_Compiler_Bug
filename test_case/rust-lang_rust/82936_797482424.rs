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
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_constant.rs at line 24:
                 assert_eq!(literal.ty, ty);
                 Constant { span, user_ty, literal: literal.into() }
             }
-            ExprKind::StaticRef { literal, .. } => Constant { span, user_ty: None, literal: literal.into() },
+            ExprKind::StaticRef { literal, .. } => {
+                Constant { span, user_ty: None, literal: literal.into() }
+            }
             ExprKind::ConstBlock { value } => {
                 Constant { span: span, user_ty: None, literal: value.into() }
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/check_match.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/usefulness.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/mod.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/const_to_pat.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/as_temp.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/as_constant.rs" "/checkout/compiler/rustc_mir_build/src/thir/arena.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
