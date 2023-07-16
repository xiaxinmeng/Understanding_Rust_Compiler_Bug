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
Diff in /checkout/compiler/rustc_middle/src/hir/map/mod.rs at line 4:
 use rustc_data_structures::fingerprint::Fingerprint;
 use rustc_data_structures::stable_hasher::{HashStable, StableHasher};
 use rustc_data_structures::svh::Svh;
-use rustc_data_structures::sync::{par_iter, ParallelIterator,par_for_each_in, Send, Sync};
+use rustc_data_structures::sync::{par_for_each_in, par_iter, ParallelIterator, Send, Sync};
 use rustc_hir::def::{DefKind, Res};
 use rustc_hir::def_id::{CrateNum, DefId, LocalDefId, CRATE_DEF_ID, LOCAL_CRATE};
 use rustc_hir::definitions::{DefKey, DefPath, DefPathHash};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/hir/place.rs" "/checkout/compiler/rustc_middle/src/hir/mod.rs" "/checkout/compiler/rustc_middle/src/lib.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/queries.rs" "/checkout/compiler/rustc_codegen_llvm/src/va_arg.rs" "/checkout/compiler/rustc_expand/src/mbe/transcribe.rs" "/checkout/compiler/rustc_expand/src/mbe/quoted.rs" "/checkout/compiler/rustc_middle/src/hir/map/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
