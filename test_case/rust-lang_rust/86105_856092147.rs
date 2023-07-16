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
Diff in /checkout/compiler/rustc_middle/src/middle/cstore.rs at line 210:
 // In order to get this left-to-right dependency ordering, we use the reverse
 // postorder of all crates putting the leaves at the right-most positions.
 pub fn used_crates(tcx: TyCtxt<'_>) -> Vec<CrateNum> {
-    tcx
-        .postorder_cnums(())
+    tcx.postorder_cnums(())
         .iter()
         .rev()
         .cloned()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/mir/visit.rs" "/checkout/compiler/rustc_middle/src/mir/coverage.rs" "/checkout/compiler/rustc_middle/src/middle/cstore.rs" "/checkout/compiler/rustc_middle/src/mir/abstract_const.rs" "/checkout/compiler/rustc_middle/src/middle/exported_symbols.rs" "/checkout/compiler/rustc_middle/src/mir/traversal.rs" "/checkout/compiler/rustc_middle/src/middle/privacy.rs" "/checkout/compiler/rustc_middle/src/middle/region.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:18
