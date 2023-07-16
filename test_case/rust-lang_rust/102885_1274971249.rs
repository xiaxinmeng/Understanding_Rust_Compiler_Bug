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
Diff in /checkout/compiler/rustc_codegen_llvm/src/back/lto.rs at line 129:
                     Err(msg) => {
                     Err(msg) => {
                         eprintln!("Couldn't add bitcode from {name}");
-                        return Err(diag_handler.fatal(&msg))
-                    },
+                        return Err(diag_handler.fatal(&msg));
                 }
             }
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs" "/checkout/compiler/rustc_codegen_llvm/src/type_.rs" "/checkout/compiler/rustc_codegen_llvm/src/intrinsic.rs" "/checkout/compiler/rustc_codegen_llvm/src/consts.rs" "/checkout/compiler/rustc_codegen_llvm/src/attributes.rs" "/checkout/compiler/rustc_codegen_llvm/src/common.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/lto.rs" "/checkout/compiler/rustc_codegen_llvm/src/coverageinfo/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
