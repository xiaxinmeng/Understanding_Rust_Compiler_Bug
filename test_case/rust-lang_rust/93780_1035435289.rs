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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 2049:
             }
 
             if !synthetic_format.is_empty() {
-                print_sidebar_block(out, "synthetic-implementations", "Auto Trait Implementations", |buf| {
-                    write_sidebar_links(buf, synthetic_format);
+                print_sidebar_block(
+                    out,
+                    "synthetic-implementations",
+                    "Auto Trait Implementations",
+                    "Auto Trait Implementations",
+                    |buf| {
+                        write_sidebar_links(buf, synthetic_format);
+                );
             }
 
 
             if !blanket_format.is_empty() {
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 2058:
-                print_sidebar_block(out, "blanket-implementations", "Blanket Implementations", |buf| {
-                    write_sidebar_links(buf, blanket_format);
+                print_sidebar_block(
+                    out,
+                    "blanket-implementations",
+                    "blanket-implementations",
+                    "Blanket Implementations",
+                    |buf| {
+                        write_sidebar_links(buf, blanket_format);
+                );
             }
         }
     }
     }
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 2209:
         "<h3 class=\"sidebar-title\">\
              <a href=\"#{}\">{}</a>\
          </h3>",
-        id,
-        title);
+        id, title
 }
 
 
 fn print_sidebar_title(buf: &mut Buffer, id: &str, title: &str) {
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 2219:
     buf.push_str("</div>");
 
 
-fn print_sidebar_block(buf: &mut Buffer, id: &str, title: &str, callback: impl FnOnce(&mut Buffer)) {
+fn print_sidebar_block(
+    buf: &mut Buffer,
+    id: &str,
+    title: &str,
+    callback: impl FnOnce(&mut Buffer),
+) {
     let mut inner_buf = Buffer::empty_from(buf);
     callback(&mut inner_buf);
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 2569:
     }
 
 
     if !sidebar.is_empty() {
-        write!(buf,
+            buf,
+            buf,
             "<section class=\"items\">\
                  <div class=\"block\">\
                      <ul>{}</ul>\
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 2576:
                  </div>\
-             </section>", sidebar);
+             </section>",
+            sidebar
     }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/format.rs" "/checkout/src/librustdoc/html/markdown/tests.rs" "/checkout/src/librustdoc/html/render/span_map.rs" "/checkout/src/librustdoc/html/render/context.rs" "/checkout/src/librustdoc/html/render/print_item.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/html/render/write_shared.rs" "/checkout/src/librustdoc/html/escape.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
