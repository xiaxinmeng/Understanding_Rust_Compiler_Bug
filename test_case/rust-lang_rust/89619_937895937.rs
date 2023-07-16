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
Diff in /checkout/compiler/rustc_middle/src/ty/vtable.rs at line 49:
     tcx: TyCtxt<'tcx>,
     key: (Ty<'tcx>, Option<ty::PolyExistentialTraitRef<'tcx>>),
-
-
     let (ty, poly_trait_ref) = key;
 
     let vtable_entries = if let Some(poly_trait_ref) = poly_trait_ref {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_query_system/src/cache.rs" "/checkout/compiler/rustc_middle/src/ty/vtable.rs" "/checkout/compiler/rustc_middle/src/ty/trait_def.rs" "/checkout/compiler/rustc_query_system/src/ich/impls_hir.rs" "/checkout/compiler/rustc_query_system/src/ich/impls_syntax.rs" "/checkout/compiler/rustc_fs_util/src/lib.rs" "/checkout/compiler/rustc_typeck/src/variance/solve.rs" "/checkout/compiler/rustc_query_system/src/dep_graph/dep_node.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
