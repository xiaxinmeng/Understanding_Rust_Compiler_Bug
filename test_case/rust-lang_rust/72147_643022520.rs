
error: cannot infer an appropriate lifetime
  --> src/lib.rs:14:34
   |
13 |   fn lookup(&self, var: &str) -> Box<dyn Iterator<Item = &E>> {
   |             ----- data with this lifetime...
14 |     Box::new(iter::once(self.map.get(var).unwrap()))
   |     -----------------------------^^^---------------- ...is captured and required to be `'static` here
   |
help: to permit non-static references in a `dyn Trait` value, you can add an explicit bound for the anonymous lifetime #1 defined on the method body at 13:3
   |
13 |   fn lookup(&self, var: &str) -> Box<dyn Iterator<Item = &E> + '_> {
   |                                                              ^^^^

error: cannot infer an appropriate lifetime
  --> src/lib.rs:19:34
   |
18 |   fn lookup(&self, var: &str) -> Box<dyn Iterator<Item = &E>> {
   |             ----- data with this lifetime...
19 |     Box::new(iter::once(self.map.get(var).unwrap()))
   |     -----------------------------^^^---------------- ...is captured and required to be `'static` here
   |
help: to permit non-static references in a `dyn Trait` value, you can add an explicit bound for the anonymous lifetime #1 defined on the method body at 18:3
   |
18 |   fn lookup(&self, var: &str) -> Box<dyn Iterator<Item = &E> + '_> {
   |                                                              ^^^^
