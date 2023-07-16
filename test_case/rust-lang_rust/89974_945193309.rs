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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_parse/src/parser/stmt.rs at line 16:
 };
 use rustc_ast::{Block, BlockCheckMode, Expr, ExprKind, Local, Stmt};
 use rustc_ast::{StmtKind, DUMMY_NODE_ID};
-use rustc_errors::{Applicability, PResult, DiagnosticBuilder};
+use rustc_errors::{Applicability, DiagnosticBuilder, PResult};
 use rustc_span::source_map::{BytePos, Span};
 use rustc_span::symbol::{kw, sym};
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_dataflow/src/rustc_peek.rs" "/checkout/compiler/rustc_parse/src/parser/item.rs" "/checkout/compiler/rustc_parse/src/parser/diagnostics.rs" "/checkout/compiler/rustc_parse/src/parser/nonterminal.rs" "/checkout/compiler/rustc_parse/src/parser/stmt.rs" "/checkout/compiler/rustc_mir_dataflow/src/storage.rs" "/checkout/compiler/rustc_parse/src/parser/mod.rs" "/checkout/compiler/rustc_parse/src/parser/attr.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
