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
Diff in /checkout/compiler/rustc_borrowck/src/borrowck_errors.rs at line 6:
 use crate::session_diagnostics::{
 use crate::session_diagnostics::{
     AssignBorrowErr, BorrowAcrossDestructor, BorrowAcrossGeneratorYield, ClosureConstructLabel,
-    InteriorDropMoveErr, MoveBorrowedErr, PathShortLive, TwoClosuresUniquelyBorrowErr, UseMutBorrowErr,
+    InteriorDropMoveErr, MoveBorrowedErr, PathShortLive, TwoClosuresUniquelyBorrowErr,
+    UseMutBorrowErr,
 
 
 impl<'cx, 'tcx> crate::MirBorrowckCtxt<'cx, 'tcx> {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_borrowck/src/lib.rs" "/checkout/compiler/rustc_borrowck/src/borrowck_errors.rs" "/checkout/compiler/rustc_borrowck/src/facts.rs" "/checkout/compiler/rustc_borrowck/src/location.rs" "/checkout/compiler/rustc_borrowck/src/dataflow.rs" "/checkout/compiler/rustc_borrowck/src/prefixes.rs" "/checkout/compiler/rustc_smir/src/very_unstable.rs" "/checkout/compiler/rustc_borrowck/src/region_infer/opaque_types.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
