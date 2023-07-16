
error: unsatisfied lifetime constraints
  --> underscore-lifetime/dyn-trait-underscore.rs:18:5
   |
16 | fn a<T>(items: &[T]) -> Box<dyn Iterator<Item=&T>> {
   |         ----- region `'1` appears in this argument
17 |     Box::new(items.iter()) //~ ERROR cannot infer an appropriate lifetime
   |     ^^^^^^^^^^^^^^^^^^^^^^ cast requires that `'1` must outlive `'static`
