
error[E0621]: explicit lifetime required in parameter type
 --> file.rs:2:27
  |
1 | fn foo<'a>((x, y): (&'a i32, &i32)) -> &'a i32 {
  |                    --------------- help: add explicit lifetime `'a` to type: `(&'a i32, &'a i32)`
2 |     if x > y { x } else { y }
  |                           ^ lifetime `'a` required
