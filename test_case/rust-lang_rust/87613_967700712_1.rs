diff
--- a/tokio-macros/src/entry.rs                                                                                           
+++ b/tokio-macros/src/entry.rs                                                                                           
@@ -293,6 +293,11 @@ fn build_config(                                                                                     
 }                                                                                                                        
                                                                                                                          
 fn parse_knobs(mut input: syn::ItemFn, is_test: bool, config: FinalConfig) -> TokenStream {                              
+    if let Some(asyncness) = input.sig.asyncness {                                                                       
+        if let Some(joined_span) = asyncness.span.join(input.sig.fn_token.span) {                                        
+            input.sig.fn_token.span = joined_span;                                                                       
+        }                                                                                                                
+    }                                                                                                                    
     input.sig.asyncness = None;                                                                                          
                                                                                                                          
     // If type mismatch occurs, the current rustc points to the last statement.                                                                                                 
