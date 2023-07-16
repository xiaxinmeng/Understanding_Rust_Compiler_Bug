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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 2259:
 }
 
 fn print_sidebar_title(buf: &mut Buffer, id: &str, title: &str) {
-        buf,
-        buf,
-        "<h3><a href=\"#{}\">{}</a></h3>",
-        id, title
-    );
+    write!(buf, "<h3><a href=\"#{}\">{}</a></h3>", id, title);
 
 fn print_sidebar_block(
 fn print_sidebar_block(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/static_files.rs" "/checkout/src/librustdoc/clean/simplify.rs" "/checkout/src/librustdoc/html/render/span_map.rs" "/checkout/src/librustdoc/html/render/print_item.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/html/render/write_shared.rs" "/checkout/src/librustdoc/html/length_limit/tests.rs" "/checkout/src/librustdoc/clean/types.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
