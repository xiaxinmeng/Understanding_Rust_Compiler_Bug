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
Diff in /checkout/compiler/rustc_resolve/src/late/lifetimes.rs at line 342:
 /// Like `resolve_lifetimes`, but does not resolve lifetimes for trait items.
 /// Also does not generate any diagnostics.
 #[tracing::instrument(level = "debug", skip(tcx))]
-fn resolve_lifetimes_trait_definition(tcx: TyCtxt<'_>, local_def_id: LocalDefId) -> ResolveLifetimes {
+fn resolve_lifetimes_trait_definition(
+    tcx: TyCtxt<'_>,
+    local_def_id: LocalDefId,
+) -> ResolveLifetimes {
     do_resolve(tcx, local_def_id, true)
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir/src/definitions.rs" "/checkout/compiler/rustc_hir/src/intravisit.rs" "/checkout/compiler/rustc_hir/src/lib.rs" "/checkout/compiler/rustc_resolve/src/late/lifetimes.rs" "/checkout/compiler/rustc_resolve/src/late/diagnostics.rs" "/checkout/compiler/rustc_resolve/src/check_unused.rs" "/checkout/compiler/rustc_resolve/src/late.rs" "/checkout/compiler/rustc_hir/src/itemlikevisit.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
