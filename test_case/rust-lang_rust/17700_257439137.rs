 nocode
error[E0053]: method `bar` has an incompatible type for trait
 --> <anon>:6:29
  |
2 |   fn bar<B>(&self, b: B) -> Self;
  |                             ---- type in trait
...
6 |   fn bar<B>(&self, b: B) -> Option<B> {
  |                             ^^^^^^^^^ expected type parameter, found a different type parameter
  |
  = note: expected type `fn(&std::option::Option<A>, B) -> std::option::Option<A>`
  = note:    found type `fn(&std::option::Option<A>, B) -> std::option::Option<B>`
