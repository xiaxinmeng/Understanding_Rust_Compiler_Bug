plain
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_parse/src/lexer/mod.rs at line 5:
 use rustc_lexer::unescape::{self, Mode};
 use rustc_lexer::{Base, DocStyle, RawStrError};
 use rustc_session::lint::builtin::{
-    TEXT_DIRECTION_CODEPOINT_IN_COMMENT, RUST_2021_PREFIXES_INCOMPATIBLE_SYNTAX,
+    RUST_2021_PREFIXES_INCOMPATIBLE_SYNTAX, TEXT_DIRECTION_CODEPOINT_IN_COMMENT,
 use rustc_session::lint::BuiltinLintDiagnostics;
 use rustc_session::parse::ParseSess;
 use rustc_session::parse::ParseSess;
Running `"/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/lexer/unicode_chars.rs" "/checkout/compiler/rustc_mir_dataflow/src/framework/cursor.rs" "/checkout/compiler/rustc_mir_dataflow/src/framework/mod.rs" "/checkout/compiler/rustc_mir_dataflow/src/framework/lattice.rs" "/checkout/compiler/rustc_mir_dataflow/src/framework/graphviz.rs" "/checkout/compiler/rustc_mir_dataflow/src/framework/tests.rs" "/checkout/compiler/rustc_mir_dataflow/src/framework/engine.rs" "/checkout/compiler/rustc_parse/src/lexer/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
