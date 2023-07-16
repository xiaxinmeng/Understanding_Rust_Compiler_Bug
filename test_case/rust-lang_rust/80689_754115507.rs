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
Diff in /checkout/compiler/rustc_ast/src/tokenstream.rs at line 280:
                         TokenTree::Delimited(span, delim, delim_tokens) => {
                             if delim == &DelimToken::Brace && !seen_braces {
                                 let mut builder = TokenStreamBuilder::new();
-                                for inner_attr in
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast/src/tokenstream.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-                                    inner_attrs.take().unwrap_or_else(|| panic!("Inner attrs already consumed for target: {:?}", target_tokens))
-                                {
+                                for inner_attr in inner_attrs.take().unwrap_or_else(|| {
+                                    panic!(
+                                        "Inner attrs already consumed for target: {:?}",
+                                        target_tokens
+                                }) {
+                                }) {
                                     builder.push(inner_attr.tokens().to_tokenstream());
                                 }
                                 builder.push(delim_tokens.clone());
Build completed unsuccessfully in 0:00:20
