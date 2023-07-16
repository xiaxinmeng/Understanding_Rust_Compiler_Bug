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
Diff in /checkout/src/librustdoc/html/highlight.rs at line 220:
         };
     });
-    write_pending_elems(
-        out,
-        out,
-        &href_context,
-        &mut pending_elems,
-        &mut current_class,
-        &closing_tags,
-    );
+    write_pending_elems(out, &href_context, &mut pending_elems, &mut current_class, &closing_tags);
 
 
 fn write_footer(out: &mut Buffer, playground_button: Option<&str>) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/tests.rs" "/checkout/src/librustdoc/html/render/search_index.rs" "/checkout/src/librustdoc/html/markdown/tests.rs" "/checkout/src/librustdoc/html/static_files.rs" "/checkout/src/librustdoc/html/format.rs" "/checkout/src/librustdoc/html/highlight.rs" "/checkout/src/librustdoc/html/highlight/tests.rs" "/checkout/src/librustdoc/html/render/context.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
