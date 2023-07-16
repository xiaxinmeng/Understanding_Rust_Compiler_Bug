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
Diff in /checkout/src/librustdoc/html/render/context.rs at line 140:
     /// Returns the `collapsed_doc_value` of the given item if this is the main crate, otherwise
     /// returns the `doc_value`.
     crate fn maybe_collapsed_doc_value<'a>(&self, item: &'a clean::Item) -> Option<String> {
-        if self.collapsed {
-            item.collapsed_doc_value()
-            item.doc_value()
-        }
-        }
+        if self.collapsed { item.collapsed_doc_value() } else { item.doc_value() }
 
 
     crate fn edition(&self) -> Edition {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/tests.rs" "/checkout/src/librustdoc/html/render/span_map.rs" "/checkout/src/librustdoc/html/render/context.rs" "/checkout/src/librustdoc/html/render/cache.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
