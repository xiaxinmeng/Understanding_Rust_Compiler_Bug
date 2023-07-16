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
Diff in /checkout/src/tools/compiletest/src/runtest.rs at line 363:
                 if self.config.target.ends_with("-fuchsia") {
                     return WillExecute::No;
-            },
-            },
-            Some(true) => {},
+            Some(true) => {}
+            Some(true) => {}
             Some(false) => return WillExecute::No,
         match self.config.mode {
         match self.config.mode {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/compiletest/src/util/tests.rs" "/checkout/src/tools/compiletest/src/json.rs" "/checkout/library/rustc-std-workspace-alloc/lib.rs" "/checkout/src/tools/compiletest/src/common.rs" "/checkout/library/rustc-std-workspace-core/lib.rs" "/checkout/src/tools/compiletest/src/runtest.rs" "/checkout/library/unwind/build.rs" "/checkout/library/core/benches/num/dec2flt/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
