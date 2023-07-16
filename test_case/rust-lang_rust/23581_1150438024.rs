
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements
  --> src/main.rs:18:14
   |
18 |         self.foo();
   |              ^^^
   |
note: first, the lifetime cannot outlive the lifetime `'b` as defined here...
  --> src/main.rs:16:9
   |
16 | impl<'a,'b> Pair<'a,'b> {
   |         ^^
note: ...but the lifetime must also be valid for the lifetime `'a` as defined here...
  --> src/main.rs:16:6
   |
16 | impl<'a,'b> Pair<'a,'b> {
   |      ^^
note: ...so that the types are compatible
  --> src/main.rs:18:14
   |
18 |         self.foo();
   |              ^^^
   = note: expected `<Pair<'a, 'b> as Foo>`
              found `<Pair<'_, '_> as Foo>`
