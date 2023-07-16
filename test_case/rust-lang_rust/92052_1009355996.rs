diff
     aliases: &[String],
 ) {
     let id = cx.derive_id(match i.inner_impl().trait_ {
-        Some(ref t) => {
-            if is_on_foreign_type {
-                get_id_for_impl_on_foreign_type(&i.inner_impl().for_, t, cx)
-            } else {
-                format!("impl-{}", small_url_encode(format!("{:#}", t.print(cx))))
-            }
-        }
+        Some(ref t) => get_id_for_impl(&i.inner_impl().for_, t, cx),
         None => "impl".to_string(),
     });
     let aliases = if aliases.is_empty() {
@@ -2194,7 +2188,7 @@ fn sidebar_struct(cx: &Context<'_>, buf: &mut Buffer, it: &clean::Item, s: &clea
     }
 }
 
-fn get_id_for_impl_on_foreign_type(
+fn get_id_for_impl(
     for_: &clean::Type,
     trait_: &clean::Path,
     cx: &Context<'_>,
@@ -2210,7 +2204,7 @@ fn extract_for_impl_name(item: &clean::Item, cx: &Context<'_>) -> Option<(String
                 // so this parameter does nothing.
                 (
                     format!("{:#}", i.for_.print(cx)),
-                    get_id_for_impl_on_foreign_type(&i.for_, trait_, cx),
+                    get_id_for_impl(&i.for_, trait_, cx),
                 )
             })
         }

