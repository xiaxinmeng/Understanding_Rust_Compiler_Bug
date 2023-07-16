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
Diff in /checkout/compiler/rustc_middle/src/ty/query/on_disk_cache.rs at line 558:
     // maps to None.
     fn compute_cnum_map(tcx: TyCtxt<'_>) -> UnhashMap<StableCrateId, CrateNum> {
         tcx.dep_graph.with_ignore(|| {
-            tcx
-                .crates(())
+            tcx.crates(())
                 .iter()
                 .chain(std::iter::once(&LOCAL_CRATE))
                 .map(|&cnum| {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_builtin_macros/src/concat.rs" "/checkout/compiler/rustc_middle/src/ty/print/pretty.rs" "/checkout/compiler/rustc_middle/src/ty/query/on_disk_cache.rs" "/checkout/compiler/rustc_middle/src/ty/print/mod.rs" "/checkout/compiler/rustc_builtin_macros/src/test.rs" "/checkout/compiler/rustc_middle/src/ty/trait_def.rs" "/checkout/compiler/rustc_builtin_macros/src/proc_macro_harness.rs" "/checkout/compiler/rustc_middle/src/ty/adjustment.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
