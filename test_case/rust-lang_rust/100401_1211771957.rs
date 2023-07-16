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
Diff in /checkout/compiler/rustc_monomorphize/src/collector.rs at line 268:
 impl<'tcx> InliningMap<'tcx> {
     fn new(pre_inlining_map: PreInlineMap<'tcx>) -> InliningMap<'tcx> {
         InliningMap {
-            index: pre_inlining_map.iter().enumerate().map(|(index, (item, _))| (*item, index)).collect(),
+            index: pre_inlining_map
+                .iter()
+                .enumerate()
+                .map(|(index, (item, _))| (*item, index))
+                .collect(),
             targets: pre_inlining_map,
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/intrinsics/type_name.rs" "/checkout/compiler/rustc_monomorphize/src/polymorphize.rs" "/checkout/compiler/rustc_incremental/src/assert_module_sources.rs" "/checkout/compiler/rustc_incremental/src/assert_dep_graph.rs" "/checkout/compiler/rustc_monomorphize/src/util.rs" "/checkout/compiler/rustc_monomorphize/src/collector.rs" "/checkout/compiler/rustc_incremental/src/persist/work_product.rs" "/checkout/compiler/rustc_const_eval/src/errors.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
