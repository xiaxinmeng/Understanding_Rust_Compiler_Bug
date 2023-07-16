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
Diff in /checkout/src/librustdoc/clean/types.rs at line 720:
                             continue;
                         }
                         // #[doc(cfg(...))]
-                        if let Some(cfg_mi) = item.meta_item().and_then(|item| rustc_expand::config::parse_cfg(&item, sess)) {
+                        if let Some(cfg_mi) = item
+                            .meta_item()
+                            .and_then(|item| rustc_expand::config::parse_cfg(&item, sess))
+                        {
                             match Cfg::parse(&cfg_mi) {
                                 Ok(new_cfg) => cfg &= new_cfg,
                                 Err(e) => sess.span_err(e.span, e.msg),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/error.rs" "/checkout/src/librustdoc/theme/tests.rs" "/checkout/src/librustdoc/visit_lib.rs" "/checkout/src/librustdoc/doctree.rs" "/checkout/src/librustdoc/clean/types.rs" "/checkout/compiler/rustc_fs_util/src/lib.rs" "/checkout/compiler/rustc_metadata/src/rmeta/mod.rs" "/checkout/src/librustdoc/externalfiles.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
