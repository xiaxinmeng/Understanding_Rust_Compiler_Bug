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
Diff in /checkout/src/tools/compiletest/src/runtest.rs at line 1791:
             child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
 
 
-        let Output { status, stdout, stderr } =
-            self.read2_abbreviated(child);
+        let Output { status, stdout, stderr } = self.read2_abbreviated(child);
         let result = ProcRes {
             status,
             status,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/compiletest/src/errors.rs" "/checkout/src/tools/compiletest/src/runtest.rs" "/checkout/src/tools/compiletest/src/header/tests.rs" "/checkout/src/tools/compiletest/src/util.rs" "/checkout/src/tools/compiletest/src/compute_diff.rs" "/checkout/src/tools/html-checker/main.rs" "/checkout/src/tools/compiletest/src/raise_fd_limit.rs" "/checkout/src/tools/compiletest/src/util/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
