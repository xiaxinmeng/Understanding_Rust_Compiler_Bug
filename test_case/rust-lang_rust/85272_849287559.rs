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
Diff in /checkout/library/core/tests/macros.rs at line 15:
 
 #[test]
 fn matches_leading_pipe() {
-    matches!(1, | 1 | 2 | 3);
+    matches!(1, |1| 2 | 3);
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/unicode.rs" "/checkout/library/core/tests/bool.rs" "/checkout/library/core/tests/slice.rs" "/checkout/library/core/tests/pattern.rs" "/checkout/library/core/tests/macros.rs" "/checkout/library/core/tests/ops/control_flow.rs" "/checkout/library/core/tests/result.rs" "/checkout/library/core/src/slice/iter/macros.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
