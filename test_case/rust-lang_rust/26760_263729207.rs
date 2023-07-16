
error[E0223]: ambiguous associated type
 --> <anon>:2:17
  |
2 |         let _ = f32::consts::PI;
  |                 ^^^^^^^^^^^^^^^ ambiguous associated type
  |
  = note: specify the type using the syntax `<f32 as Trait>::consts`
