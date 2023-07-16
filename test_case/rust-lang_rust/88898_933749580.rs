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
Diff in /checkout/compiler/rustc_codegen_llvm/src/debuginfo/mod.rs at line 38:
 use libc::c_uint;
 use smallvec::SmallVec;
 use std::cell::RefCell;
-use std::rc::Rc;
 use std::iter;
+use std::rc::Rc;
 use tracing::debug;
 
 mod create_scope_map;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/utils.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/gdb.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/mod.rs" "/checkout/compiler/rustc_save_analysis/src/lib.rs" "/checkout/compiler/rustc_save_analysis/src/dumper.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/create_scope_map.rs" "/checkout/compiler/rustc_incremental/src/assert_module_sources.rs" "/checkout/compiler/rustc_save_analysis/src/sig.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
