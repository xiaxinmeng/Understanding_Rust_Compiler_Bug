
error[E0689]: can't call method `powi` on ambiguous numeric type `{float}`
  --> $DIR/method-on-ambiguous-numeric-type.rs:12:17
   |
LL |     let x = 2.0.powi(2);
   |                 ^^^^
help: you must specify a concrete type for this numeric value, like `f32`
   |
LL |     let x = 2.0_f32.powi(2);
   |             ^^^^^^^
