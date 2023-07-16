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
Diff in /checkout/compiler/rustc_parse/src/parser/attr.rs at line 203:
                     .span_label(prev_attr_sp, prev_attr_note);
 
 
-            diag.note("inner attributes, like `#![no_std]`, annotate the item enclosing them, and \
-                are usually found at the beginning of source files");
+            diag.note(
+                "inner attributes, like `#![no_std]`, annotate the item enclosing them, and \
+                are usually found at the beginning of source files",
+            );
             if self.annotate_following_item_if_applicable(&mut diag, attr_sp, false).is_some() {
                 diag.note("outer attributes, like `#[test]`, annotate the item following them");
             };
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/parser/mod.rs" "/checkout/compiler/rustc_parse/src/parser/path.rs" "/checkout/compiler/rustc_parse/src/parser/generics.rs" "/checkout/compiler/rustc_parse/src/parser/expr.rs" "/checkout/compiler/rustc_serialize/src/json.rs" "/checkout/compiler/rustc_serialize/src/opaque.rs" "/checkout/compiler/rustc_expand/src/proc_macro_server.rs" "/checkout/compiler/rustc_parse/src/parser/attr.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
