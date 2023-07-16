plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/src/tools/compiletest/src/runtest.rs at line 3487:
         // changes.
         // eg. $SRC_DIR/libcore/mem.rs:323:14 becomes $SRC_DIR/libcore/mem.rs:LL:COL
         lazy_static! {
-            static ref SRC_DIR_RE: Regex = Regex::new("SRC_DIR(.+):\\d+:\\d+(: \\d+:\\d+)?")
-                .unwrap();
+            static ref SRC_DIR_RE: Regex =
+                Regex::new("SRC_DIR(.+):\\d+:\\d+(: \\d+:\\d+)?").unwrap();
 
 
         normalized = SRC_DIR_RE.replace_all(&normalized, "SRC_DIR$1:LL:COL").into_owned();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/compiletest/src/runtest.rs" "/checkout/src/tools/compiletest/src/tests.rs" "/checkout/src/etc/test-float-parse/src/bin/long-fractions.rs" "/checkout/src/tools/compiletest/src/raise_fd_limit.rs" "/checkout/src/tools/compiletest/src/main.rs" "/checkout/src/tools/compiletest/src/runtest/debugger.rs" "/checkout/src/tools/compiletest/src/runtest/tests.rs" "/checkout/src/etc/test-float-parse/src/bin/short-decimals.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
