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
Diff in /checkout/src/librustdoc/clean/types.rs at line 518:
                                     format!("{}/std/", s.trim_end_matches('/'))
                                 }
                                 Some(ExternalLocation::Unknown) | None => {
-                                    format!("https://doc.rust-lang.org/{}/std/", crate::doc_rust_lang_org_channel())
+                                    format!(
+                                        "https://doc.rust-lang.org/{}/std/",
+                                        crate::doc_rust_lang_org_channel()
                                 }
                             };
                             };
                             // This is a primitive so the url is done "by hand".
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/cfg/tests.rs" "/checkout/src/librustdoc/clean/auto_trait.rs" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/clean/types.rs" "/checkout/src/librustdoc/doctree.rs" "/checkout/src/librustdoc/passes/unindent_comments.rs" "/checkout/src/librustdoc/clean/utils/tests.rs"` failed.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
