diff
  macro straw_man() {
+     use $call_site::S;
      fn f() {
-         println!("{:?}", #S);
+         println!("{:?}", S);
      }
}
