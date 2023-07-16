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
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 580:
             DefKind::Struct | DefKind::Union => {
                 // While structs and unions have type privacy, their fields do not.
                 if vis.is_public() {
-                    let item =
-                        self.tcx.hir().expect_item(def_id);
+                    let item = self.tcx.hir().expect_item(def_id);
                     if let hir::ItemKind::Struct(ref struct_def, _)
                     | hir::ItemKind::Union(ref struct_def, _) = item.kind
                     {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_interface/src/passes.rs" "/checkout/compiler/rustc_interface/src/callbacks.rs" "/checkout/compiler/rustc_privacy/src/lib.rs" "/checkout/compiler/rustc_query_impl/src/values.rs" "/checkout/compiler/rustc_query_impl/src/on_disk_cache.rs" "/checkout/compiler/rustc_query_impl/src/profiling_support.rs" "/checkout/compiler/rustc_query_impl/src/lib.rs" "/checkout/compiler/rustc_interface/src/proc_macro_decls.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
