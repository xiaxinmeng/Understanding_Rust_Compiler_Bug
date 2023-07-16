
error[E0599]: no method named `box_and_set_data` found for struct `BoxHolder<Box<for<'r> fn(&'r str) -> &'r str {a}>>` in the current scope
  --> src/main.rs:18:9
   |
1  | pub struct BoxHolder<BoxedFn> {
   | ----------------------------- method `box_and_set_data` not found for this
...
18 |     foo.box_and_set_data(a);
   |         ^^^^^^^^^^^^^^^^ method not found in `BoxHolder<Box<for<'r> fn(&'r str) -> &'r str {a}>>`
