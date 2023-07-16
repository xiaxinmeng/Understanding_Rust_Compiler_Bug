
error[E0517]: attribute should be applied to a struct, enum, function, or union
  --> src/main.rs:11:12
   |
11 |       #[repr(align(1))]
   |              ^^^^^^^^
12 | /     fn foo() {
13 | |         println!("bar");
14 | |     }
   | |_____- not a struct, enum, function, or union

For more information about this error, try `rustc --explain E0517`.
error: could not compile `asdf` due to previous error
