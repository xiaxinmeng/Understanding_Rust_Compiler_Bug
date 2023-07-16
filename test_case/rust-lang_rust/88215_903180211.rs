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
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links/early.rs at line 1:
 use rustc_ast as ast;
 use rustc_hir::def::Namespace::TypeNS;
-use rustc_hir::def_id::{CRATE_DEF_ID, LocalDefId};
+use rustc_hir::def_id::{LocalDefId, CRATE_DEF_ID};
 use rustc_interface::interface;
 use rustc_span::Span;
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/formats/item_type.rs" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/passes/collect_intra_doc_links/early.rs" "/checkout/src/librustdoc/passes/collect_trait_impls.rs" "/checkout/src/librustdoc/config.rs" "/checkout/library/alloc/benches/vec_deque_append.rs" "/checkout/src/librustdoc/passes/strip_priv_imports.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
