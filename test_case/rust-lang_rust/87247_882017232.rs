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
Diff in /checkout/library/test/src/formatters/pretty.rs at line 4:
 use crate::{
     bench::fmt_bench_samples,
     console::{ConsoleTestState, OutputLocation},
     term,
+    test_result::TestResult,
     time,
     types::TestDesc,
     types::TestDesc,
 };
Diff in /checkout/library/test/src/console.rs at line 13:
     formatters::{JsonFormatter, JunitFormatter, OutputFormatter, PrettyFormatter, TerseFormatter},
     helpers::{concurrency::get_concurrency, metrics::MetricMap},
     options::{Options, OutputFormat},
-    term,
+    run_tests, term,
     test_result::TestResult,
     test_result::TestResult,
     time::{TestExecTime, TestSuiteExecTime},
     types::{NamePadding, TestDesc, TestDescAndFn},
Diff in /checkout/library/test/src/formatters/terse.rs at line 4:
 use crate::{
     bench::fmt_bench_samples,
     console::{ConsoleTestState, OutputLocation},
     term,
+    test_result::TestResult,
     time,
     types::NamePadding,
     types::NamePadding,
     types::TestDesc,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/fmt/float.rs" "/checkout/compiler/rustc_ast_lowering/src/item.rs" "/checkout/library/test/src/stats/tests.rs" "/checkout/library/test/src/tests.rs" "/checkout/library/test/src/formatters/pretty.rs" "/checkout/library/test/src/formatters/mod.rs" "/checkout/library/test/src/formatters/junit.rs" "/checkout/compiler/rustc_ast_lowering/src/asm.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
