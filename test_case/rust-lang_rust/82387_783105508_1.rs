
error[E0308]: mismatched types
  --> src/main.rs:11:9
   |
11 |     foo(bar());
   |         ^^^^^ expected struct `Foo`, found enum `Result`
   |
   = note: expected struct `Foo`
                found enum `Result<Foo, ()>`
help: you can get a `Foo` from a `Result<Foo, ()>` 
   |
11 |     foo(bar().unwrap());
   |              ^^^^^^^^^
