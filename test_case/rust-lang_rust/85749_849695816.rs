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
Diff in /checkout/src/librustdoc/core.rs at line 6:
 use rustc_feature::UnstableFeatures;
 use rustc_hir::def::Namespace::TypeNS;
 use rustc_hir::def::Res;
-use rustc_hir::def_id::{DefId, CRATE_DEF_INDEX, LocalDefId};
+use rustc_hir::def_id::{DefId, LocalDefId, CRATE_DEF_INDEX};
 use rustc_hir::HirId;
 use rustc_hir::{
     intravisit::{self, NestedVisitorMap, Visitor},
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/src/librustdoc/markdown.rs" "/checkout/src/librustdoc/externalfiles.rs" "/checkout/src/librustdoc/lint.rs" "/checkout/src/librustdoc/core.rs" "/checkout/src/librustdoc/json/conversions.rs" "/checkout/src/librustdoc/html/format.rs" "/checkout/src/librustdoc/clean/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
