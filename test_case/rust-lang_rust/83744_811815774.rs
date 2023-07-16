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
Diff in /checkout/compiler/rustc_expand/src/config.rs at line 342:
 
                 let attr = attr::mk_attr_from_item(item, tokens, attr.style, span);
                 if attr.has_name(sym::crate_type) {
-                    self.sess.span_err(attr.span, "`crate_type` is not allowed within a `#![cfg_attr]`");
+                    self.sess
+                        .span_err(attr.span, "`crate_type` is not allowed within a `#![cfg_attr]`");
                 }
                 if attr.has_name(sym::crate_name) {
-                    self.sess.span_err(attr.span, "`crate_name` is not allowed within a `#![cfg_attr]`");
+                    self.sess
+                        .span_err(attr.span, "`crate_name` is not allowed within a `#![cfg_attr]`");
                 self.process_cfg_attr(attr)
             })
             })
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_span/src/def_id.rs" "/checkout/compiler/rustc_data_structures/src/sso/map.rs" "/checkout/compiler/rustc_passes/src/lang_items.rs" "/checkout/compiler/rustc_parse/src/lib.rs" "/checkout/compiler/rustc_parse/src/validate_attr.rs" "/checkout/compiler/rustc_expand/src/build.rs" "/checkout/compiler/rustc_expand/src/config.rs" "/checkout/compiler/rustc_span/src/edition.rs"` failed.
Build completed unsuccessfully in 0:00:16
Build completed unsuccessfully in 0:00:16
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
