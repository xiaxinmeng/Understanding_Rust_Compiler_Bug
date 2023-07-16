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
Diff in /checkout/src/librustdoc/passes/check_code_block_syntax.rs at line 1:
 //! Validates syntax inside Rust code blocks (\`\`\`rust).
 use rustc_data_structures::sync::{Lock, Lrc};
 use rustc_errors::{
-    emitter::Emitter, translation::{to_fluent_args, Translate}, Applicability, Diagnostic, Handler,
-    LazyFallbackBundle,
+    emitter::Emitter,
+    translation::{to_fluent_args, Translate},
+    Applicability, Diagnostic, Handler, LazyFallbackBundle,
 use rustc_parse::parse_stream_from_source_str;
 use rustc_session::parse::ParseSess;
 use rustc_session::parse::ParseSess;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/auto_trait.rs" "/checkout/src/librustdoc/passes/check_code_block_syntax.rs" "/checkout/src/librustdoc/clean/render_macro_matchers.rs" "/checkout/src/librustdoc/passes/calculate_doc_coverage.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/passes/stripper.rs" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/src/librustdoc/passes/propagate_doc_cfg.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
