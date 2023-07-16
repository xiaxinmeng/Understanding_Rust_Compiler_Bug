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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/tools/compiletest/src/header.rs at line 112:
                 }
 
                 if config.target_panic == PanicStrategy::Abort
-                    && config.parse_name_directive(ln, "needs-unwind") {
+                    && config.parse_name_directive(ln, "needs-unwind")
                     props.ignore = true;
                 }
 
Diff in /checkout/src/tools/compiletest/src/main.rs at line 5:
Diff in /checkout/src/tools/compiletest/src/main.rs at line 5:
 
 extern crate test;
 
-use crate::common::{PanicStrategy, UI_EXTENSIONS, expected_output_path, output_base_dir, output_relative_path};
+use crate::common::{
+    expected_output_path, output_base_dir, output_relative_path, PanicStrategy, UI_EXTENSIONS,
+};
 use crate::common::{CompareMode, Config, Debugger, Mode, PassMode, Pretty, TestPaths};
 use crate::util::logv;
 use getopts::Options;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/compiletest/src/header.rs" "/checkout/src/tools/compiletest/src/common.rs" "/checkout/src/tools/compiletest/src/runtest.rs" "/checkout/src/tools/compiletest/src/tests.rs" "/checkout/src/tools/compiletest/src/util.rs" "/checkout/src/tools/compiletest/src/header/tests.rs" "/checkout/src/tools/compiletest/src/read2.rs" "/checkout/src/tools/compiletest/src/util/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
