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
Diff in /checkout/compiler/rustc_const_eval/src/transform/check_consts/qualifs.rs at line 351:
     // FIXME(valtrees): check whether const qualifs should behave the same
     // way for type and mir constants.
     let uneval = match constant.literal {
-        ConstantKind::Ty(ct) if matches!(ct.kind(), ty::ConstKind::Param(_) | ty::ConstKind::Error(_)) => None,
+        ConstantKind::Ty(ct)
+            if matches!(ct.kind(), ty::ConstKind::Param(_) | ty::ConstKind::Error(_)) =>
+            None
+        }
+        }
         ConstantKind::Ty(c) => bug!("expected ConstKind::Param here, found {:?}", c),
         ConstantKind::Unevaluated(uv, _) => Some(uv),
         ConstantKind::Val(..) => None,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/check.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/resolver.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/ops.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/qualifs.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/mod.rs" "/checkout/compiler/rustc_const_eval/src/transform/mod.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/error.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/post_drop_elaboration.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
