
error: casting `&{float}` as `f32` is invalid
  --> $DIR/cast-rfc0401.rs:81:30
   |
81 |     vec![0.0].iter().map(|s| s as f32).collect::<Vec<f32>>();
   |                              ^^^^^^^^ cannot cast `&{float}` as `f32`
   |
help: did you mean `*s`?
  --> $DIR/cast-rfc0401.rs:81:30
   |
81 |     vec![0.0].iter().map(|s| s as f32).collect::<Vec<f32>>();
   |                              ^
