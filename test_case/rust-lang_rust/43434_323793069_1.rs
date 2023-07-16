
11 | fn foo<'a>((x, y): (&'a i32, &i32)) -> &'a i32 {
   |                     ------------- consider changing type to `(&'a i32, &'a i32)`
12 |     if x > y { x } else { y }
   |                           ^ lifetime `'a` required
