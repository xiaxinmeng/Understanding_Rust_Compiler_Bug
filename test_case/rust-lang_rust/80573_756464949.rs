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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 13:
     PerNS,
 };
 use rustc_hir::def_id::{CrateNum, DefId};
-use rustc_middle::{bug, ty};
 use rustc_middle::ty::TyCtxt;
+use rustc_middle::{bug, ty};
 use rustc_resolve::ParentScope;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 use rustc_session::lint::{
     builtin::{BROKEN_INTRA_DOC_LINKS, PRIVATE_INTRA_DOC_LINKS},
Build completed unsuccessfully in 0:00:19
