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
Diff in /checkout/src/librustdoc/html/markdown.rs at line 621:
                                 is_paragraph = true;
                             }
                             html::push_html(&mut ret, content.into_iter());
-                            write!(ret, "&nbsp;<a href=\"#fnref{}\">↩</a>", id)
-                                .unwrap();
+                            write!(ret, "&nbsp;<a href=\"#fnref{}\">↩</a>", id).unwrap();
                             if is_paragraph {
                                 ret.push_str("</p>");
                             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/etc/test-float-parse/_common.rs" "/checkout/src/librustdoc/html/markdown.rs" "/checkout/src/librustdoc/html/tests.rs" "/checkout/src/librustdoc/html/escape.rs" "/checkout/src/librustdoc/html/mod.rs" "/checkout/src/librustdoc/html/markdown/tests.rs" "/checkout/src/librustdoc/html/highlight/tests.rs" "/checkout/src/librustdoc/html/toc.rs"` failed.
Build completed unsuccessfully in 0:00:14
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
