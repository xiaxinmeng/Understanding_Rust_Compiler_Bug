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
Diff in /checkout/src/librustdoc/html/highlight/fixtures/sample.rs at line 10:
     mac!(foo, &mut bar);
     assert!(self.length < N && index <= self.length);
     ::std::env::var("gateau").is_ok();
-    let s:std::path::PathBuf = std::path::PathBuf::new();
+    let s: std::path::PathBuf = std::path::PathBuf::new();
     let mut s = String::new();
     match &s {
     match &s {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/highlight/tests.rs" "/checkout/src/librustdoc/visit_ast.rs" "/checkout/src/librustdoc/formats/renderer.rs" "/checkout/src/librustdoc/formats/item_type.rs" "/checkout/src/librustdoc/html/highlight/fixtures/sample.rs" "/checkout/src/librustdoc/formats/cache.rs" "/checkout/src/librustdoc/visit_lib.rs" "/checkout/src/librustdoc/html/markdown.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
