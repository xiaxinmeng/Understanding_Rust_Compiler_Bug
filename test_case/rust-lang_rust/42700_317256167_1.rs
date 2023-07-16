
// error[E0611]: explicit lifetime required in the type of `self`
//    |
// 20 |     fn foo<'a>(&self, mut x: Vec<&'a Foo>) {
//    |                ^^^^^ consider changing to `&'a self`
// ...
// 25 |         x.push(self);
//    |                 ---- lifetime `'a` required
