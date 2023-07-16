diff
--- a/compiler/rustc_error_messages/locales/en-US/infer.ftl
+++ b/compiler/rustc_error_messages/locales/en-US/infer.ftl
@@ -268,28 +268,28 @@ infer_but_calling_introduces = {$has_param_name ->
     [true] `{$param_name}`
     *[false] `fn` parameter
 } has {$lifetime_kind ->
-    [named] lifetime `{lifetime}`
-    *[anon] an anonymous lifetime `'_`
-} but calling `{assoc_item}` introduces an implicit `'static` lifetime requirement
+    [true] lifetime `{$lifetime}`
+    *[false] an anonymous lifetime `'_`
+} but calling `{$assoc_item}` introduces an implicit `'static` lifetime requirement
     .label1 = {$has_lifetime ->
-        [named] lifetime `{lifetime}`
-        *[anon] an anonymous lifetime `'_`
+        [true] lifetime `{$lifetime}`
+        *[false] an anonymous lifetime `'_`
     }
     .label2 = ...is used and required to live as long as `'static` here because of an implicit lifetime bound on the {$has_impl_path ->
-        [named] `impl` of `{$impl_path}`
-        *[anon] inherent `impl`
+        [true] `impl` of `{$impl_path}`
+        *[false] inherent `impl`
     }
