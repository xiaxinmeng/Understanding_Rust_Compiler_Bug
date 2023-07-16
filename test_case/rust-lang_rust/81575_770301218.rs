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
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 1328:
                                 if let Some(res) =
                                     self.check_full_res(new_ns, path_str, base_node, extra_fragment)
-                                    kind =
-                                    kind =
-                                        ResolutionFailure::WrongNamespace { res, expected_ns };
+                                    kind = ResolutionFailure::WrongNamespace { res, expected_ns };
                                     break;
                             }
                             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:17
