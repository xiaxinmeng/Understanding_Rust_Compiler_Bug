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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_transmute/src/layout/tree.rs at line 284:
                 }
 
                 ty::Array(ty, len) => {
-                    let len = len.try_eval_usize(tcx, ParamEnv::reveal_all()).ok_or(Err::Unspecified)?;
+                    let len =
+                        len.try_eval_usize(tcx, ParamEnv::reveal_all()).ok_or(Err::Unspecified)?;
                     let elt = Tree::from_ty(*ty, tcx)?;
                     Ok(std::iter::repeat(elt)
                         .take(len as usize)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_cranelift/build_system/utils.rs" "/checkout/compiler/rustc_codegen_cranelift/build_system/build_backend.rs" "/checkout/compiler/rustc_codegen_cranelift/build_system/tests.rs" "/checkout/compiler/rustc_codegen_cranelift/build_system/prepare.rs" "/checkout/compiler/rustc_codegen_cranelift/build_system/config.rs" "/checkout/compiler/rustc_codegen_cranelift/build_system/mod.rs" "/checkout/compiler/rustc_transmute/src/layout/tree.rs" "/checkout/compiler/rustc_codegen_cranelift/build_system/abi_cafe.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
