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
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1527:
                 if render_method_item {
                     let id = cx.derive_id(format!("{}.{}", item_type, name));
                     let source_id = trait_
-                        .and_then(|trait_| trait_
-                            .items.iter()
-                            .find(|item| item.name.map(|n| n.as_str().eq(&name.as_str())).unwrap_or(false))
-                        ).map(|item| format!("{}.{}", item.type_(), name));
-                    write!(w, "<h4 id=\"{}\" class=\"{}{}{}\">", id, item_type, extra_class, in_trait_class);
+                        .and_then(|trait_| {
+                            trait_.items.iter().find(|item| {
+                                item.name.map(|n| n.as_str().eq(&name.as_str())).unwrap_or(false)
+                        })
+                        })
+                        .map(|item| format!("{}.{}", item.type_(), name));
+                        w,
+                        w,
+                        "<h4 id=\"{}\" class=\"{}{}{}\">",
+                        id, item_type, extra_class, in_trait_class
+                    );
                     w.write_str("<code>");
                     render_assoc_item(
                         w,
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1538:
                         item,
                         link.anchor(source_id.as_ref().unwrap_or(&id)),
                         ItemType::Impl,
+                        cx,
                     );
                     );
                     w.write_str("</code>");
                     render_stability_since_raw(
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1556:
             clean::TypedefItem(ref tydef, _) => {
                 let source_id = format!("{}.{}", ItemType::AssocType, name);
                 let id = cx.derive_id(source_id.clone());
-                write!(w, "<h4 id=\"{}\" class=\"{}{}{}\"><code>", id, item_type, extra_class, in_trait_class);
+                    w,
+                    w,
+                    "<h4 id=\"{}\" class=\"{}{}{}\"><code>",
+                    id, item_type, extra_class, in_trait_class
+                );
                     w,
                     item,
                     item,
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1573:
             clean::AssocConstItem(ref ty, ref default) => {
                 let source_id = format!("{}.{}", item_type, name);
                 let id = cx.derive_id(source_id.clone());
-                write!(w, "<h4 id=\"{}\" class=\"{}{}{}\"><code>", id, item_type, extra_class, in_trait_class);
+                    w,
+                    w,
+                    "<h4 id=\"{}\" class=\"{}{}{}\"><code>",
+                    id, item_type, extra_class, in_trait_class
+                );
                     w,
                     item,
                     item,
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1581:
                     default.as_ref(),
                     link.anchor(if trait_.is_some() { &source_id } else { &id }),
-                    cx
+                    cx,
                 );
                 );
                 w.write_str("</code>");
                 render_stability_since_raw(
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1598:
             clean::AssocTypeItem(ref bounds, ref default) => {
                 let source_id = format!("{}.{}", item_type, name);
                 let id = cx.derive_id(source_id.clone());
-                write!(w, "<h4 id=\"{}\" class=\"{}{}{}\"><code>", id, item_type, extra_class, in_trait_class);
+                    w,
+                    w,
+                    "<h4 id=\"{}\" class=\"{}{}{}\"><code>",
+                    id, item_type, extra_class, in_trait_class
+                );
                     w,
                     item,
                     item,
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1606:
                     default.as_ref(),
                     link.anchor(if trait_.is_some() { &source_id } else { &id }),
                     "",
-                    cx.cache()
+                    cx.cache(),
                 );
                 w.write_str("</code>");
                 write!(w, "<a href=\"#{}\" class=\"anchor\"></a>", id);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/context.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/html/render/cache.rs" "/checkout/src/librustdoc/html/render/print_item.rs" "/checkout/src/librustdoc/html/render/write_shared.rs" "/checkout/src/librustdoc/html/toc.rs" "/checkout/src/librustdoc/html/format.rs" "/checkout/src/librustdoc/html/render/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
