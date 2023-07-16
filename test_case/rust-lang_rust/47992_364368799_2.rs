diff
- macro straw_man() {
-     struct #Exported;
-     struct Internal;
+ macro straw_man() $Internal:ident {
+     struct Exported;
+     struct $Internal;
  }
