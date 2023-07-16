
error: cannot infer an appropriate lifetime
 --> src/main.rs:2:16
  |
1 | fn foo<T>(x: &Vec<T>) -> Box<dyn Iterator<Item=&T>> {
  |              ------- data with this lifetime...
2 |     Box::new(x.iter())
  |     -----------^^^^--- ...is captured and required to be `'static` here
  |
help: to permit non-static references in a `dyn Trait` value, you can add an explicit bound for the anonymous lifetime #1 defined on the function body at 1:1
  |
1 | fn foo<T>(x: &Vec<T>) -> Box<dyn Iterator<Item=&T> + '_> {
  |                                                    ^^^^
