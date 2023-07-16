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
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 54:
 use serde::ser::SerializeSeq;
 use serde::{Serialize, Serializer};
+use crate::clean::utils::print_const_expr;
+use crate::clean::utils::print_const_expr;
 use crate::clean::{self, FakeDefId, GetDefId, RenderedLink, SelfTy};
 use crate::docfs::PathError;
 use crate::error::Error;
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 66:
     Buffer, PrintWithSpace,
 };
 use crate::html::markdown::{Markdown, MarkdownHtml, MarkdownSummaryLine};
-use crate::clean::utils::print_const_expr;
 
 /// A pair of name and its optional document.
 crate type NameDoc = (String, Option<String>);
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1368:
                     w.write_str("</h4>");
             }
             }
-            clean::ConstantItem(clean::Constant { type_: ref ty, kind: clean::ConstantKind::Local { body, .. } }) => {
+            clean::ConstantItem(clean::Constant {
+                type_: ref ty,
+                kind: clean::ConstantKind::Local { body, .. },
+            }) => {
                 let item_type = ItemType::AssocConst;
                 let source_id = format!("{}.{}", item_type, name);
                 let id = cx.derive_id(source_id.clone());
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/html/render/print_item.rs" "/checkout/src/librustdoc/html/render/write_shared.rs" "/checkout/src/librustdoc/html/render/cache.rs" "/checkout/src/librustdoc/core.rs" "/checkout/src/bootstrap/job.rs" "/checkout/src/librustdoc/visit_ast.rs" "/checkout/src/librustdoc/html/render/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
