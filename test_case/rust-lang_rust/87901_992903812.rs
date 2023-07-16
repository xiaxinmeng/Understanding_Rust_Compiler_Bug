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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_parse/src/parser/item.rs at line 1518:
                 };
                 // We use `parse_fn` to get a span for the function
                 let fn_parse_mode = FnParseMode { req_name: |_| true, req_body: true };
-                if let Err(mut db) = self.parse_fn(&mut Vec::new(), fn_parse_mode, lo, &inherited_vis) {
+                if let Err(mut db) =
+                    self.parse_fn(&mut Vec::new(), fn_parse_mode, lo, &inherited_vis)
+                {
                     db.delay_as_bug();
                 let mut err = self.struct_span_err(
                 let mut err = self.struct_span_err(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/parser/item.rs" "/checkout/compiler/rustc_parse/src/parser/path.rs" "/checkout/compiler/rustc_expand/src/config.rs" "/checkout/compiler/rustc_interface/src/util.rs" "/checkout/compiler/rustc_interface/src/tests.rs" "/checkout/compiler/rustc_interface/src/lib.rs" "/checkout/compiler/rustc_expand/src/base.rs" "/checkout/compiler/rustc_parse/src/parser/nonterminal.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
