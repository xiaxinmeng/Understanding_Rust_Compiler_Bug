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
Diff in /checkout/library/std/src/io/mod.rs at line 2241:
         SizeHint::lower_bound(&self.first) + SizeHint::lower_bound(&self.second)
 
 
-    fn upper_bound(&self) -> Option<usize > {
+    fn upper_bound(&self) -> Option<usize> {
         match (SizeHint::upper_bound(&self.first), SizeHint::upper_bound(&self.second)) {
             (Some(first), Some(second)) => Some(first + second),
             _ => None,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/io/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/library/std/src/io/mod.rs at line 2248:
     }
 }
-
 
 
 /// Reader adaptor which limits the bytes read from an underlying reader.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:19
