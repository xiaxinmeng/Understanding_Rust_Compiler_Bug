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
Diff in /checkout/compiler/rustc_interface/src/util.rs at line 1:
-use info;
 use crate::errors;
+use info;
 use libloading::Library;
 use rustc_ast as ast;
 use rustc_codegen_ssa::traits::CodegenBackend;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_interface/src/queries.rs" "/checkout/compiler/rustc_interface/src/interface.rs" "/checkout/compiler/rustc_interface/src/tests.rs" "/checkout/compiler/rustc_lint/src/lib.rs" "/checkout/compiler/rustc_interface/src/util.rs" "/checkout/compiler/rustc_interface/src/errors.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
