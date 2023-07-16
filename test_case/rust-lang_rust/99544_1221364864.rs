plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/str/lossy.rs at line 265:
 #[unstable(feature = "utf8_chunks", issue = "99543")]
 impl fmt::Debug for Utf8Chunks<'_> {
     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
-        f.debug_struct("Utf8Chunks")
-            .field("source", &self.debug())
-            .finish()
+        f.debug_struct("Utf8Chunks").field("source", &self.debug()).finish()
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/iter/adapters/array_chunks.rs" "/checkout/library/core/src/ops/generator.rs" "/checkout/library/core/src/ops/unsize.rs" "/checkout/library/core/src/ops/mod.rs" "/checkout/library/core/src/ops/control_flow.rs" "/checkout/library/core/src/ops/deref.rs" "/checkout/library/core/src/ops/arith.rs" "/checkout/library/core/src/str/lossy.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
