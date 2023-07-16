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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_parse/src/parser/expr.rs at line 236:
             }
 
             if op.node == AssocOp::Less {
-                if self.token.kind == token::Gt && self.prev_token.span.hi() == self.token.span.lo() {
+                if self.token.kind == token::Gt && self.prev_token.span.hi() == self.token.span.lo()
+                {
                     // Look for PHP's `<>` and recover
                     let sp = op.span.to(self.token.span);
                     self.struct_span_err(sp, "invalid comparison operator `<>`")
Diff in /checkout/compiler/rustc_parse/src/parser/expr.rs at line 254:
                     self.struct_span_err(sp, &format!("invalid comparison operator `<=>`"))
                         .span_suggestion_short(
                             sp,
-                            &format!("`<=>` is not a valid comparison operator, use `std::cmp::Ordering`"),
+                            &format!(
+                                "`<=>` is not a valid comparison operator, use `std::cmp::Ordering`"
+                            ),
                             "<=>".to_string(),
                             Applicability::Unspecified,
                         )
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_dataflow/src/lib.rs" "/checkout/compiler/rustc_parse_format/src/tests.rs" "/checkout/compiler/rustc_parse_format/src/lib.rs" "/checkout/compiler/rustc_parse/src/parser/expr.rs" "/checkout/compiler/rustc_parse/src/parser/generics.rs" "/checkout/compiler/rustc_parse/src/parser/stmt.rs" "/checkout/compiler/rustc_parse/src/parser/attr.rs" "/checkout/compiler/rustc_mir_dataflow/src/move_paths/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
