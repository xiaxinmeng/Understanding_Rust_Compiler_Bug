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
Diff in /checkout/src/librustdoc/html/markdown.rs at line 523:
 }
 
 fn is_forbidden_tag(t: &Tag<'_>) -> bool {
-    matches!(
-        t,
-        Tag::CodeBlock(_) | Tag::Table(_) | Tag::TableHead | Tag::TableRow | Tag::TableCell
-    )
+    matches!(t, Tag::CodeBlock(_) | Tag::Table(_) | Tag::TableHead | Tag::TableRow | Tag::TableCell)
 
 
 impl<'a, I: Iterator<Item = Event<'a>>> Iterator for SummaryLine<'a, I> {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/tests.rs" "/checkout/src/librustdoc/html/toc/tests.rs" "/checkout/src/librustdoc/html/highlight/tests.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/html/highlight/fixtures/sample.rs" "/checkout/src/librustdoc/html/render/cache.rs" "/checkout/src/librustdoc/html/sources.rs" "/checkout/src/librustdoc/html/markdown.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
