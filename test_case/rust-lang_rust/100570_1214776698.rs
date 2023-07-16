plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_parse/src/parser/attr_wrapper.rs at line 418:
                 let delimited = AttrAnnotatedTokenTree::Delimited(dspan, delim, stream);
                     .last_mut()
                     .last_mut()
-                    .unwrap_or_else(|| {
-                        panic!("Bottom token frame is missing for token: {token:?}")
-                    })
+                    .unwrap_or_else(|| panic!("Bottom token frame is missing for token: {token:?}"))
                     .inner
                     .push((delimited, Spacing::Alone));
             }
Diff in /checkout/compiler/rustc_parse/src/lexer/unescape_error_reporting.rs at line 214:
             };
             let c = escaped_char(c);
-            handler
-            handler
-                .struct_span_err(span, &format!("{msg}: `{c}`"))
-                .span_label(span, msg)
-                .emit();
+            handler.struct_span_err(span, &format!("{msg}: `{c}`")).span_label(span, msg).emit();
         }
         EscapeError::NonAsciiCharInByte => {
             assert!(mode.is_bytes());
Diff in /checkout/compiler/rustc_parse/src/lexer/unescape_error_reporting.rs at line 246:
                 utf8.push(c);
                 err.span_suggestion(
-                    &format!(
-                    &format!(
-                        "if you meant to use the UTF-8 encoding of {c:?}, use \\xHH escapes"
-                    ),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/validate_attr.rs" "/checkout/compiler/rustc_parse/src/parser/nonterminal.rs" "/checkout/compiler/rustc_parse/src/lexer/mod.rs" "/checkout/compiler/rustc_parse/src/parser/diagnostics.rs" "/checkout/compiler/rustc_parse/src/lexer/unicode_chars.rs" "/checkout/compiler/rustc_parse/src/parser/item.rs" "/checkout/compiler/rustc_parse/src/lexer/unescape_error_reporting.rs" "/checkout/compiler/rustc_error_codes/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+                    &format!("if you meant to use the UTF-8 encoding of {c:?}, use \\xHH escapes"),
                     utf8.as_bytes()
                         .iter()
                         .map(|b: &u8| format!("\\x{:X}", *b))
Diff in /checkout/compiler/rustc_parse/src/lexer/unescape_error_reporting.rs at line 280:
         EscapeError::LeadingUnderscoreUnicodeEscape => {
             let (c, span) = last_char();
             let msg = "invalid start of unicode escape";
-            handler
-                .struct_span_err(span, &format!("{msg}: `{c}`"))
-                .span_label(span, msg)
-                .emit();
+            handler.struct_span_err(span, &format!("{msg}: `{c}`")).span_label(span, msg).emit();
         }
         EscapeError::OverlongUnicodeEscape => {
Build completed unsuccessfully in 0:00:12
