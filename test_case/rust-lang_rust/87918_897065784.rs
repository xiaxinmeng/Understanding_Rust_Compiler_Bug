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
Diff in /checkout/compiler/rustc_codegen_llvm/src/back/write.rs at line 411:
         .pgo_sample_use
         .as_ref()
         .map(|path_buf| CString::new(path_buf.to_string_lossy().as_bytes()).unwrap())
 }
 
 
 pub(crate) fn should_use_new_llvm_pass_manager(config: &ModuleConfig) -> bool {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/allocator.rs" "/checkout/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/write.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/profiling.rs" "/checkout/compiler/rustc_codegen_llvm/src/coverageinfo/mod.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/lto.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/archive.rs" "/checkout/compiler/rustc_codegen_llvm/src/callee.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
