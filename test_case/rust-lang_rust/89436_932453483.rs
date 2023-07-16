
error[E0271]: type mismatch resolving `for<'a> <&str as MiniYokeable<'_>>::Output == <&str as MiniYokeable<'a>>::Output`
  --> test.rs:51:5
   |
51 |     map_project_broken(bar, |bar, _| bar.string_1)
   |     ^^^^^^^^^^^^^^^^^^ expected associated type, found `&str`
   |
   = note: expected associated type `<&str as MiniYokeable<'a>>::Output`
                    found reference `&str`
   = help: consider constraining the associated type `<&str as MiniYokeable<'a>>::Output` to `&str` or calling a method that returns `<&str as MiniYokeable<'a>>::Output`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
note: required by a bound in `map_project_broken`
  --> test.rs:14:10
   |
9  | fn map_project_broken<Y, P>(
   |    ------------------ required by a bound in this
...
14 |     ) -> <P as MiniYokeable<'a>>::Output,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `map_project_broken`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
