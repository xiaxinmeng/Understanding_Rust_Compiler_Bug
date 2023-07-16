plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/src/librustdoc/html/markdown.rs at line 541:
             for event in &mut self.inner {
                 match &event.0 {
                     Event::End(Tag::Heading(..)) => break,
-                    | Event::Text(text)
-                    | Event::Code(text) => {
+                    Event::Text(text) | Event::Code(text) => {
                         id.extend(text.chars().filter_map(slugify));
                         self.buf.push_back(event);
                     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/highlight/fixtures/sample.rs" "/checkout/src/librustdoc/html/highlight/tests.rs" "/checkout/src/librustdoc/html/url_parts_builder.rs" "/checkout/src/librustdoc/html/markdown.rs" "/checkout/src/librustdoc/html/length_limit.rs" "/checkout/src/librustdoc/html/sources.rs" "/checkout/src/librustdoc/html/toc/tests.rs" "/checkout/src/librustdoc/html/highlight/fixtures/union.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
