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
Diff in /checkout/compiler/rustc_typeck/src/coherence/inherent_impls.rs at line 137:
         }
 
 
-        if self.tcx.has_attr(def_id, sym::rustc_has_incoherent_inherent_impls)
-        {
+        if self.tcx.has_attr(def_id, sym::rustc_has_incoherent_inherent_impls) {
             let hir::ItemKind::Impl(hir::Impl { items, .. }) = item.kind else {
                 bug!("expected `impl` item: {:?}", item);
             };
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_build.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/record_consumed_borrow.rs" "/checkout/compiler/rustc_typeck/src/coherence/inherent_impls_overlap.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_visualize.rs" "/checkout/compiler/rustc_typeck/src/coherence/builtin.rs" "/checkout/compiler/rustc_typeck/src/coherence/orphan.rs" "/checkout/compiler/rustc_typeck/src/coherence/inherent_impls.rs" "/checkout/compiler/rustc_typeck/src/coherence/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
