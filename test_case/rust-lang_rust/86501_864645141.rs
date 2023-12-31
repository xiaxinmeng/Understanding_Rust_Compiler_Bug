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
Diff in /checkout/src/librustdoc/doctest.rs at line 3:
 use rustc_data_structures::sync::Lrc;
 use rustc_errors::{ColorConfig, ErrorReported};
 use rustc_hir as hir;
-use rustc_hir::intravisit;
 use rustc_hir::def_id::LOCAL_CRATE;
+use rustc_hir::intravisit;
 use rustc_hir::{HirId, CRATE_HIR_ID};
 use rustc_interface::interface;
 use rustc_middle::hir::map::Map;
Diff in /checkout/src/librustdoc/doctest.rs at line 11:
 use rustc_middle::ty::TyCtxt;
 use rustc_session::config::{self, CrateType, ErrorOutputType};
 use rustc_session::{lint, DiagnosticOutput, Session};
 use rustc_span::edition::Edition;
 use rustc_span::source_map::SourceMap;
 use rustc_span::symbol::sym;
Diff in /checkout/src/librustdoc/doctest.rs at line 18:
Diff in /checkout/src/librustdoc/doctest.rs at line 18:
+use rustc_span::Symbol;
 use rustc_span::{BytePos, FileName, Pos, Span, DUMMY_SP};
 use rustc_target::spec::TargetTriple;
 use tempfile::Builder as TempFileBuilder;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/doctest.rs" "/checkout/src/librustdoc/visit_lib.rs" "/checkout/src/librustdoc/lib.rs" "/checkout/src/tools/unstable-book-gen/src/main.rs" "/checkout/src/librustdoc/fold.rs" "/checkout/src/librustdoc/visit_ast.rs" "/checkout/library/panic_unwind/src/dwarf/eh.rs" "/checkout/library/panic_unwind/src/hermit.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
