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
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 2899:
             cx,
             &cx.root_path(),
             highlight::DecorationInfo(decoration_info),
-            sources::SourceContext::Embedded { url: &call_data.url, offset: line_min, needs_expansion },
+            sources::SourceContext::Embedded {
+                url: &call_data.url,
+                offset: line_min,
+                needs_expansion,
         );
         );
         write!(w, "</div></div>");
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/context.rs" "/checkout/src/librustdoc/html/render/print_item.rs" "/checkout/src/librustdoc/error.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/tools/tidy/src/error_codes_check.rs" "/checkout/src/tools/tidy/src/main.rs" "/checkout/src/tools/tidy/src/deps.rs" "/checkout/src/librustdoc/html/render/write_shared.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
