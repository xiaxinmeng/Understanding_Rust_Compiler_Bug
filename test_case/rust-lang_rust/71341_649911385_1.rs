
error[E0759]: cannot infer an appropriate lifetime
  --> src/lib.rs:14:9
   |
13 | fn use_it<'a>(val: &'a dyn ObjectTrait) -> impl OtherTrait<'a> + 'a {
   |                    ------------------- this data with lifetime `'a`...
14 |     val.use_self()
   |         ^^^^^^^^ ...is captured and required to live as long as `'static` here
