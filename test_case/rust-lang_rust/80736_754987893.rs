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
Diff in /checkout/library/std/src/backtrace.rs at line 410:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/backtrace.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 
 impl LazilyResolvedCapture {
     fn new(capture: Capture) -> Self {
-        LazilyResolvedCapture {
-            sync: Once::new(),
-            capture: UnsafeCell::new(capture),
-        }
+        LazilyResolvedCapture { sync: Once::new(), capture: UnsafeCell::new(capture) }
 
 
     fn force(&self) -> &Capture {
Build completed unsuccessfully in 0:00:22
