
error[E0308]: mismatched types
  --> src/main.rs:9:30
   |
9  | fn not_all_paths(a: &str) -> u32 {
   |    -------------             ^^^ expected u32, found ()
   |    |
   |    this function's body doesn't return
...
13 |     }; // "not all control paths return a value" due to ;
   |      - help: consider removing this semicolon
   |
   = note: expected type `u32`
              found type `()`

error[E0308]: match arms have incompatible types
  --> src/main.rs:24:5
   |
24 | /     match c {
25 | |         "baz" => Box::new(Baz),
26 | |         _ => Box::new(Bar),
   | |              ------------- match arm with an incompatible type
27 | |     }; // "match arms have incompatible types" due to ;
   | |_____^ expected struct `Baz`, found struct `Bar`
   |
   = note: expected type `std::boxed::Box<Baz>`
              found type `std::boxed::Box<Bar>`

error[E0308]: mismatched types
  --> src/main.rs:23:22
   |
23 | fn wrong(c: &str) -> Box<Foo> {
   |    -----             ^^^^^^^^ expected struct `std::boxed::Box`, found ()
   |    |
   |    this function's body doesn't return
...
27 |     }; // "match arms have incompatible types" due to ;
   |      - help: consider removing this semicolon
   |
   = note: expected type `std::boxed::Box<(dyn Foo + 'static)>`
              found type `()`
