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
Diff in /checkout/src/librustdoc/html/render/write_shared.rs at line 499:
Diff in /checkout/src/librustdoc/clean/types.rs at line 40:
                 } else {
                 } else {
                     Some(Implementor {
-                        text: imp
-                            .inner_impl()
-                            .print(false, cx)
-                            .to_string(),
+                        text: imp.inner_impl().print(false, cx).to_string(),
                         synthetic: imp.inner_impl().synthetic,
                         types: collect_paths_for_type(imp.inner_impl().for_.clone(), cx.cache()),
 use crate::core::DocContext;
 use crate::formats::cache::Cache;
 use crate::formats::item_type::ItemType;
-use crate::html::render::Context;
-use crate::html::render::Context;
 use crate::html::render::cache::ExternalLocation;
+use crate::html::render::Context;
 
 use self::FnRetTy::*;
 use self::ItemKind::*;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/fold.rs" "/checkout/src/librustdoc/clean/types.rs" "/checkout/src/librustdoc/lib.rs" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/src/librustdoc/theme.rs" "/checkout/src/librustdoc/doctree.rs" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/src/librustdoc/clean/utils.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
