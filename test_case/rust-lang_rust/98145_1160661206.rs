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
Diff in /checkout/compiler/rustc_mir_dataflow/src/move_paths/un_derefer.rs at line 1:
 use rustc_data_structures::fx::FxHashMap;
 use rustc_middle::mir::*;
-use rustc_middle::ty::{TyCtxt};
+use rustc_middle::ty::TyCtxt;
 
 /// Used for reverting changes made by DerefSeparator
 pub struct UnDerefer<'tcx> {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_dataflow/src/move_paths/un_derefer.rs" "/checkout/compiler/rustc_mir_dataflow/src/move_paths/builder.rs" "/checkout/compiler/rustc_mir_dataflow/src/move_paths/abs_domain.rs" "/checkout/compiler/rustc_mir_dataflow/src/move_paths/mod.rs" "/checkout/compiler/rustc_mir_dataflow/src/lib.rs" "/checkout/compiler/rustc_interface/src/lib.rs" "/checkout/compiler/rustc_mir_dataflow/src/rustc_peek.rs" "/checkout/compiler/rustc_interface/src/callbacks.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
