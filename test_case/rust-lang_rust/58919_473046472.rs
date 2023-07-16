
help: you can add an explicit named lifetime `'a` to the type of `x` and the return type
   |
LL | fn elided<'a>(x: &'a i32) -> impl Copy + 'a { x }
   |          ^^^^     ^^                   ^^^^
