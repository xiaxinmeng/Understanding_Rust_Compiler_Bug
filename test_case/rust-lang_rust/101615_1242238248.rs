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
Diff in /checkout/compiler/rustc_query_impl/src/on_disk_cache.rs at line 798:
 }
 
-
-
 impl<'a, 'tcx> Decodable<CacheDecoder<'a, 'tcx>> for &'tcx FxHashMap<DefId, Ty<'tcx>> {
     fn decode(d: &mut CacheDecoder<'a, 'tcx>) -> Self {
         RefDecodable::decode(d)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_query_impl/src/plumbing.rs" "/checkout/compiler/rustc_query_impl/src/lib.rs" "/checkout/compiler/rustc_query_impl/src/on_disk_cache.rs" "/checkout/compiler/rustc_incremental/src/assert_module_sources.rs" "/checkout/compiler/rustc_incremental/src/lib.rs" "/checkout/compiler/rustc_incremental/src/assert_dep_graph.rs" "/checkout/compiler/rustc_incremental/src/persist/fs/tests.rs" "/checkout/compiler/rustc_query_impl/src/keys.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
