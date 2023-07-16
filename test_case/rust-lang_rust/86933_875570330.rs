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
Diff in /checkout/src/librustdoc/html/static_files.rs at line 155:
 crate mod noto_sans_kr {
     /// The file `noto-sans-kr-v13-korean-regular.woff`, the Regular variant of the Noto Sans KR
     /// font.
-    crate static REGULAR: &[u8] = include_bytes!("static/fonts/noto-sans-kr-v13-korean-regular.woff");
+    crate static REGULAR: &[u8] =
+        include_bytes!("static/fonts/noto-sans-kr-v13-korean-regular.woff");
 
     /// The file `noto-sans-kr-v13-korean-regular-LICENSE.txt`, the license text of the Noto Sans KR
     /// font.
Build completed unsuccessfully in 0:00:11
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/highlight/tests.rs" "/checkout/src/librustdoc/html/toc/tests.rs" "/checkout/src/librustdoc/html/highlight/fixtures/sample.rs" "/checkout/src/librustdoc/html/render/cache.rs" "/checkout/src/librustdoc/html/sources.rs" "/checkout/src/librustdoc/html/render/write_shared.rs" "/checkout/src/librustdoc/html/static_files.rs" "/checkout/src/librustdoc/html/render/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
