diff
diff --git a/src/librustdoc/html/render/print_item.rs b/src/librustdoc/html/render/print_item.rs
index f6fd0f7a467..7227d816b9d 100644
--- a/src/librustdoc/html/render/print_item.rs
+++ b/src/librustdoc/html/render/print_item.rs
@@ -947,8 +947,8 @@ fn print_tuple_struct_fields(w: &mut Buffer, cx: &Context<'_>, s: &[clean::Item]
             w.write_str(",&nbsp;");
         }
         match *ty.kind {
-            clean::StrippedItem(box clean::StructFieldItem(..)) => w.write_str("_"),
-            clean::StructFieldItem(ref ty) => write!(w, "{}", ty.print(cx)),
+            clean::StrippedItem(box clean::StructFieldItem(ref ty))
+            | clean::StructFieldItem(ref ty) => write!(w, "{}", ty.print(cx)),
             _ => unreachable!(),
         }
     }
@@ -1066,24 +1066,27 @@ fn item_enum(w: &mut Buffer, cx: &Context<'_>, it: &clean::Item, e: &clean::Enum
                     name = variant.name.as_ref().unwrap(),
                 );
                 for field in fields {
-                    use crate::clean::StructFieldItem;
-                    if let StructFieldItem(ref ty) = *field.kind {
-                        let id = cx.derive_id(format!(
-                            "variant.{}.field.{}",
-                            variant.name.as_ref().unwrap(),
-                            field.name.as_ref().unwrap()
-                        ));
-                        write!(
-                            w,
-                            "<span id=\"{id}\" class=\"variant small-section-header\">\
-                                 <a href=\"#{id}\" class=\"anchor field\"></a>\
-                                 <code>{f}:&nbsp;{t}</code>\
-                             </span>",
-                            id = id,
-                            f = field.name.as_ref().unwrap(),
-                            t = ty.print(cx)
-                        );
-                        document(w, cx, field, Some(variant));
+                    match *field.kind {
+                        clean::StrippedItem(box clean::StructFieldItem(ref ty))
+                        | clean::StructFieldItem(ref ty) => {
+                            let id = cx.derive_id(format!(
+                                "variant.{}.field.{}",
+                                variant.name.as_ref().unwrap(),
+                                field.name.as_ref().unwrap()
+                            ));
+                            write!(
+                                w,
+                                "<span id=\"{id}\" class=\"variant small-section-header\">\
+                                <a href=\"#{id}\" class=\"anchor field\"></a>\
+                                <code>{f}:&nbsp;{t}</code>\
+                                </span>",
+                                id = id,
+                                f = field.name.as_ref().unwrap(),
+                                t = ty.print(cx)
+                            );
+                            document(w, cx, field, Some(variant));
+                        }
+                        _ => unreachable!(),
                     }
                 }
                 w.write_str("</div></div>");
