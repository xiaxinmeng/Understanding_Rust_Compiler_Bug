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
Diff in /checkout/compiler/rustc_llvm/build.rs at line 2:
 use std::path::{Path, PathBuf};
 
 
-use build_helper::{output, tracked_env_var_os, maybe_static_clang_rt};
+use build_helper::{maybe_static_clang_rt, output, tracked_env_var_os};
 
 fn detect_llvm_link() -> (&'static str, &'static str) {
     // Force the link mode we want, preferring static by default, but
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/post_drop_elaboration.rs" "/checkout/compiler/rustc_parse_format/src/tests.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/mod.rs" "/checkout/compiler/rustc_llvm/build.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/ops.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/qualifs.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/resolver.rs" "/checkout/compiler/rustc_parse_format/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
