
error[E0308]: mismatched types
 --> src/main.rs:2:9
  |
2 |     bar(f);
  |         ^ expected concrete lifetime, found bound lifetime parameter 'b
  |
  = note: expected type `fn(&'b (), &'a ()) -> &'a &'b ()`
             found type `fn(&'a (), &'b ()) -> &'a &'b ()`
