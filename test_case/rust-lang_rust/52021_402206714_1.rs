
error: unsatisfied lifetime constraints
  --> underscore-lifetime/dyn-trait-underscore.rs:18:5
   |
16 | fn a<T>(items: &[T]) -> Box<dyn Iterator<Item=&T>> {
   |         -----           -------------------------- lifetime bound defaults to `'static`
   |         |
   |         the fully elaborated type of `items` is `&'1 [T]`
17 |     Box::new(items.iter()) //~ ERROR cannot infer an appropriate lifetime
   |     ^^^^^^^^^^^^^^^^^^^^^^ cast requires that `'1` must outlive `'static`
