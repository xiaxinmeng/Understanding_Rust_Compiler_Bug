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
Diff in /checkout/compiler/rustc_metadata/src/creader.rs at line 131:
 impl CStore {
 impl CStore {
     pub fn from_tcx(tcx: TyCtxt<'_>) -> &CStore {
-        tcx.cstore_untracked().as_any().downcast_ref::<CStore>().expect("`tcx.cstore` is not a `CStore`")
+        tcx.cstore_untracked()
+            .as_any()
+            .downcast_ref::<CStore>()
+            .expect("`tcx.cstore` is not a `CStore`")
 
     fn alloc_new_crate_num(&mut self) -> CrateNum {
     fn alloc_new_crate_num(&mut self) -> CrateNum {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_resolve/src/late/lifetimes.rs" "/checkout/compiler/rustc_resolve/src/late/diagnostics.rs" "/checkout/compiler/rustc_attr/src/lib.rs" "/checkout/compiler/rustc_metadata/src/rmeta/mod.rs" "/checkout/compiler/rustc_metadata/src/locator.rs" "/checkout/compiler/rustc_metadata/src/creader.rs" "/checkout/compiler/rustc_metadata/src/rmeta/decoder.rs" "/checkout/compiler/rustc_resolve/src/build_reduced_graph.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
