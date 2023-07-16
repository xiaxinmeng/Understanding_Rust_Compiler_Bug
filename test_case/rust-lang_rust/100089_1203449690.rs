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
########                                                                  11.4%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-07-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/mir/visit.rs at line 992:
 
 macro_rules! basic_blocks {
 macro_rules! basic_blocks {
-    ($body:ident, mut) => (
+    ($body:ident, mut) => {
         if Self::AVOID_INVALIDATING_CFG {
             $body.basic_blocks.as_mut_preserves_cfg()
         } else {
Diff in /checkout/compiler/rustc_middle/src/mir/visit.rs at line 999:
             $body.basic_blocks.as_mut()
-    );
-    );
-    ($body:ident,) => ($body.basic_blocks());
+    };
+    ($body:ident,) => {
+        $body.basic_blocks()
 }
 
 macro_rules! basic_blocks_iter {
 macro_rules! basic_blocks_iter {
Diff in /checkout/compiler/rustc_middle/src/mir/visit.rs at line 1006:
-    ($body:ident, mut) => (
+    ($body:ident, mut) => {
         basic_blocks!($body, mut).iter_enumerated_mut()
-    );
-    ($body:ident,) => (
+    };
+    ($body:ident,) => {
         basic_blocks!($body,).iter_enumerated()
+    };
 }
 
 
 macro_rules! visit_place_fns {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/mir/mono.rs" "/checkout/compiler/rustc_middle/src/mir/visit.rs" "/checkout/compiler/rustc_middle/src/mir/terminator.rs" "/checkout/compiler/rustc_middle/src/mir/type_visitable.rs" "/checkout/compiler/rustc_middle/src/mir/tcx.rs" "/checkout/compiler/rustc_middle/src/mir/coverage.rs" "/checkout/compiler/rustc_middle/src/mir/spanview.rs" "/checkout/compiler/rustc_middle/src/mir/switch_sources.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
