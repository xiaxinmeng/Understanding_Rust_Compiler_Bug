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
Diff in /checkout/src/librustdoc/passes/collect_trait_impls.rs at line 146:
                         if cleaner.keep_impl_with_def_id(for_did.into()) {
                             let mut targets = FxHashSet::default();
                             targets.insert(for_did);
-                            add_deref_target(cx, &type_did_to_deref_target, &mut cleaner, &mut targets, for_did);
+                            add_deref_target(
+                                cx,
+                                &type_did_to_deref_target,
+                                &mut cleaner,
+                                &mut targets,
+                                for_did,
                         }
                     }
                 }
                 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/strip_private.rs" "/checkout/src/librustdoc/passes/propagate_doc_cfg.rs" "/checkout/src/librustdoc/passes/strip_hidden.rs" "/checkout/src/librustdoc/passes/html_tags.rs" "/checkout/src/librustdoc/passes/calculate_doc_coverage.rs" "/checkout/src/librustdoc/passes/collect_trait_impls.rs" "/checkout/src/librustdoc/scrape_examples.rs" "/checkout/src/librustdoc/passes/bare_urls.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
