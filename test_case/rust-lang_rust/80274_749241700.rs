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
Diff in /checkout/compiler/rustc_lint/src/levels.rs at line 12:
 use rustc_middle::hir::map::Map;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lint/src/levels.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 use rustc_middle::lint::LevelSource;
 use rustc_middle::lint::LintDiagnosticBuilder;
-use rustc_middle::lint::{struct_lint_level, LintLevelMap, LintLevelSets, LintSet, LintLevelSource};
+use rustc_middle::lint::{
+    struct_lint_level, LintLevelMap, LintLevelSets, LintLevelSource, LintSet,
+};
 use rustc_middle::ty::query::Providers;
 use rustc_middle::ty::TyCtxt;
 use rustc_session::lint::{builtin, Level, Lint, LintId};
Build completed unsuccessfully in 0:00:20
