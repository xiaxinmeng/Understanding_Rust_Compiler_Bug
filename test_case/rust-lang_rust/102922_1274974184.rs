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
Diff in /checkout/compiler/rustc_errors/src/json.rs at line 319:
                 message: translated_message.to_string(),
                 code: None,
                 level: "help",
-                spans: Self::filter_empty_spans_with_no_replacement(DiagnosticSpan::from_suggestion(sugg, &args, je)),
+                spans: Self::filter_empty_spans_with_no_replacement(
+                    DiagnosticSpan::from_suggestion(sugg, &args, je),
+                ),
                 children: vec![],
                 rendered: None,
Diff in /checkout/compiler/rustc_errors/src/json.rs at line 373:
     }
 
 
     fn filter_empty_spans_with_no_replacement(spans: Vec<DiagnosticSpan>) -> Vec<DiagnosticSpan> {
-        spans.into_iter()
-            .filter(|span| !(span.byte_start == span.byte_end && span.suggested_replacement == Some(String::new())))
+            .into_iter()
+            .into_iter()
+            .filter(|span| {
+                !(span.byte_start == span.byte_end
+                    && span.suggested_replacement == Some(String::new()))
             .collect()
     }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_errors/src/snippet.rs" "/checkout/compiler/rustc_errors/src/json.rs" "/checkout/compiler/rustc_errors/src/annotate_snippet_emitter_writer.rs" "/checkout/compiler/rustc_errors/src/translation.rs" "/checkout/compiler/rustc_errors/src/lock.rs" "/checkout/compiler/rustc_errors/src/json/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
