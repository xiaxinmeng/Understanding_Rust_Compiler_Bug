
--- a/src/lib.rs	2020-01-16 09:29:57.000000000 -0800
+++ b/src/lib.rs	2020-11-04 10:57:27.000000000 -0800
@@ -163,10 +163,10 @@
             quote!(builder.field(&wrapper))
         };
         quote! {
-             let builder = {
+             {
                  let wrapper = #wrapper;
                  #call
-             };
+             }
         }
     });
     let debug_builder = if is_struct {
