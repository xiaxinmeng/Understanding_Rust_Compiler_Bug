
error[E0308]: mismatched types
  --> src/main.rs:9:34
   |
9  |   fn not_all_paths(a: &str) -> u32 {
   |  __________________________________^
10 | |     match a {
11 | |         "baz" => 0,
12 | |         _ => 1,
13 | |     }; // not all control paths return a value due to ;
   | |      - help: consider removing this semicolon
14 | | }
   | |_^ expected u32, found ()
   |
   = note: expected type `u32`
              found type `()`

error[E0308]: match arms have incompatible types
  --> src/main.rs:24:5
   |
24 | /     match c {
25 | |         "baz" => Box::new(Baz),
26 | |         _ => Box::new(Bar),
27 | |     }; // match arms have incompatible types due to ;
   | |_____^ expected struct `Baz`, found struct `Bar`
   |
   = note: expected type `std::boxed::Box<Baz>`
              found type `std::boxed::Box<Bar>`
note: match arm with an incompatible type
  --> src/main.rs:26:14
   |
26 |         _ => Box::new(Bar),
   |              ^^^^^^^^^^^^^

error[E0308]: mismatched types
  --> src/main.rs:23:31
   |
23 |   fn wrong(c: &str) -> Box<Foo> {
   |  _______________________________^
24 | |     match c {
25 | |         "baz" => Box::new(Baz),
26 | |         _ => Box::new(Bar),
27 | |     }; // match arms have incompatible types due to ;
   | |      - help: consider removing this semicolon
28 | | }
   | |_^ expected struct `std::boxed::Box`, found ()
   |
   = note: expected type `std::boxed::Box<Foo + 'static>`
              found type `()`
