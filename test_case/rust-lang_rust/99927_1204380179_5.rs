
error[E0308]: mismatched types
   --> src/main.rs:17:15
    |
 17 |         func: &foo, //~ ERROR mismatched types
    |               ^^^^ expected fn pointer, found fn item
    |
    = note: expected reference `&fn() -> Option<isize>`
               found reference `&[fn item {foo}: fn() -> Option<isize>]`
