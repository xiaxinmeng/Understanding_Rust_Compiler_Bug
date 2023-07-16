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
Diff in /checkout/src/librustdoc/html/markdown.rs at line 1323:
             | LinkType::ShortcutUnknown),
             _,
             _,
-        )) = ev.0 {
+        )) = ev.0
+        {
             debug!("found link: {dest}");
             let span = span_for_link(&dest, ev.1);
             filter_map(MarkdownLink { kind, link: dest.into_string(), range: span })
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/sources.rs" "/checkout/src/librustdoc/html/length_limit.rs" "/checkout/src/librustdoc/html/url_parts_builder.rs" "/checkout/src/librustdoc/html/highlight/fixtures/union.rs" "/checkout/src/librustdoc/html/highlight/fixtures/sample.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/clean/utils/tests.rs" "/checkout/src/librustdoc/html/markdown.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
