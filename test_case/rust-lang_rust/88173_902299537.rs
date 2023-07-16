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
Diff in /checkout/src/librustdoc/html/length_limit.rs at line 1:
 //! See [`HtmlWithLimit`].
-use std::ops::ControlFlow;
 use std::fmt::Write;
 use std::fmt::Write;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/json/mod.rs" "/checkout/src/librustdoc/html/render/span_map.rs" "/checkout/src/librustdoc/html/render/context.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/html/render/print_item.rs" "/checkout/src/librustdoc/html/highlight/tests.rs" "/checkout/src/librustdoc/html/render/write_shared.rs" "/checkout/src/librustdoc/html/length_limit.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+use std::ops::ControlFlow;
 
 use crate::html::escape::Escape;
 
Diff in /checkout/src/librustdoc/html/length_limit.rs at line 29:
 impl HtmlWithLimit {
     /// Create a new buffer, with a limit of `length_limit`.
     pub(super) fn new(length_limit: usize) -> Self {
-        Self { buf: String::new(), len: 0, limit: length_limit, unclosed_tags: Vec::new(), queued_tags: Vec::new() }
+            buf: String::new(),
+            len: 0,
+            limit: length_limit,
+            unclosed_tags: Vec::new(),
+            unclosed_tags: Vec::new(),
+            queued_tags: Vec::new(),
     }
 
 
     /// Finish using the buffer and get the written output.
