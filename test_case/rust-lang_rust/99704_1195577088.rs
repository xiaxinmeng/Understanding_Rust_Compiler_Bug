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

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-07-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_const_eval/src/transform/check_consts/ops.rs at line 23:
 use super::ConstCx;
 use crate::errors::{
 use crate::errors::{
-    MutDerefErr, NonConstOpErr, PanicNonStrErr, RawPtrToIntErr,
-    StaticAccessErr, TransientMutBorrowErr, TransientMutBorrowErrRaw,
+    MutDerefErr, NonConstOpErr, PanicNonStrErr, RawPtrToIntErr, StaticAccessErr,
+    TransientMutBorrowErr, TransientMutBorrowErrRaw,
 };
 use crate::util::{call_kind, CallDesugaringKind, CallKind};
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/post_drop_elaboration.rs" "/checkout/compiler/rustc_monomorphize/src/polymorphize.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/mod.rs" "/checkout/compiler/rustc_monomorphize/src/lib.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/ops.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/check.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/resolver.rs" "/checkout/compiler/rustc_monomorphize/src/collector.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
