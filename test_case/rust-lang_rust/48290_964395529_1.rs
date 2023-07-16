test
error[E0308]: `if` and `else` have incompatible types
  --> src/main.rs:9:9
   |
6  | /     if  timestamp % 2 == 0 {
7  | |         variant1()
   | |         ---------- expected because of this
8  | |     } else {
9  | |         variant2()
   | |         ^^^^^^^^^^ expected opaque type, found a different opaque type
10 | |     }
   | |_____- `if` and `else` have incompatible types
...
17 |   fn variant2() -> impl ToString {
   |                    ------------- the found opaque type
   |
   = note:     expected type `impl ToString` (opaque type at <src/main.rs:13:18>)
           found opaque type `impl ToString` (opaque type at <src/main.rs:17:18>)
   = note: distinct uses of `impl Trait` result in different opaque types
help: you could change the return type to be a boxed trait object
   |
3  | fn get_name() -> Box<dyn ToString> {
   |                  ~~~~~~~         +
help: if you change the return type to expect trait objects, box the returned expressions
   |
7  ~         Box::new(variant1())
8  |     } else {
9  ~         Box::new(variant2())
   |

For more information about this error, try `rustc --explain E0308`.
error: could not compile `e0308` due to previous error
