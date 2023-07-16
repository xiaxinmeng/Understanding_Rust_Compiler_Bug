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
             handler
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/lexer/tokentrees.rs" "/checkout/compiler/rustc_parse/src/lib.rs" "/checkout/compiler/rustc_errors/src/registry.rs" "/checkout/compiler/rustc_errors/src/json.rs" "/checkout/compiler/rustc_errors/src/annotate_snippet_emitter_writer.rs" "/checkout/compiler/rustc_errors/src/emitter.rs" "/checkout/compiler/rustc_errors/src/snippet.rs" "/checkout/compiler/rustc_parse/src/lexer/unescape_error_reporting.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
