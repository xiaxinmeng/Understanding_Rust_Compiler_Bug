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
-                            span_bug!(
-                                constant.span,
-                                "codegen encountered silent error",
-                            );
+                            span_bug!(constant.span, "codegen encountered silent error",);
                     }
                 }
                 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_cranelift/src/allocator.rs" "/checkout/compiler/rustc_codegen_cranelift/src/trap.rs" "/checkout/compiler/rustc_codegen_cranelift/src/inline_asm.rs" "/checkout/compiler/rustc_codegen_cranelift/src/constant.rs" "/checkout/compiler/rustc_codegen_cranelift/build_sysroot/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
