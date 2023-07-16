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
###########################################################               82.1%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_monomorphize/src/collector.rs at line 180:
 //! regardless of whether it is actually needed or not.
 
 use rustc_data_structures::fx::{FxHashMap, FxHashSet};
-use rustc_data_structures::sync::{MTLock, MTRef, par_for_each_in};
+use rustc_data_structures::sync::{par_for_each_in, MTLock, MTRef};
 use rustc_hir as hir;
 use rustc_hir::def::DefKind;
 use rustc_hir::def_id::{DefId, DefIdMap, LocalDefId};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_index/src/bit_set/tests.rs" "/checkout/compiler/rustc_monomorphize/src/lib.rs" "/checkout/compiler/rustc_index/src/vec/tests.rs" "/checkout/compiler/rustc_index/src/interval/tests.rs" "/checkout/compiler/rustc_monomorphize/src/util.rs" "/checkout/compiler/rustc_monomorphize/src/collector.rs" "/checkout/compiler/rustc_traits/src/normalize_projection_ty.rs" "/checkout/compiler/rustc_index/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread 'thread '<unnamed><unnamed>' panicked at '' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channeltx.send(entry.into_path()) failed with sending on a closed channel', ', format.rsformat.rs::166166::1717
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
Build completed unsuccessfully in 0:00:14
