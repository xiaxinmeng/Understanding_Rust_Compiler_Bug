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
################################################                          67.2%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/ty/abstract_const.rs at line 1:
 //! A subset of a mir body used for const evaluatability checking.
 use crate::mir;
-use crate::ty::{self, SubstsRef, Ty, TyCtxt, EarlyBinder, DelaySpanBugEmitted, subst::Subst};
+use crate::ty::{self, subst::Subst, DelaySpanBugEmitted, EarlyBinder, SubstsRef, Ty, TyCtxt};
 use rustc_errors::ErrorGuaranteed;
-use std::ops::ControlFlow;
 use std::iter;
+use std::ops::ControlFlow;
 rustc_index::newtype_index! {
 rustc_index::newtype_index! {
     /// An index into an `AbstractConst`.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/abstract_const.rs" "/checkout/compiler/rustc_middle/src/ty/structural_impls.rs" "/checkout/compiler/rustc_middle/src/ty/consts.rs" "/checkout/compiler/rustc_middle/src/ty/adt.rs" "/checkout/compiler/rustc_middle/src/ty/fast_reject.rs" "/checkout/compiler/rustc_middle/src/ty/visit.rs" "/checkout/compiler/rustc_middle/src/ty/error.rs" "/checkout/compiler/rustc_middle/src/ty/codec.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'thread 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:167:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rsthread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
Build completed unsuccessfully in 0:00:15
