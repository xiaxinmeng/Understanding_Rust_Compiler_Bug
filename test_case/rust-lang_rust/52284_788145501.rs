
error[E0308]: mismatched types
  --> src/main.rs:12:9
   |
11 | /     if arr1.len() == 0 && arr2.len() == 0 {
12 | |         Err("No elements".to_owned())
   | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found enum `Result`
13 | |     }
   | |_____- expected this to be `()`
   |
   = note: expected unit type `()`
                   found enum `Result<_, String>`
help: consider using a semicolon here
   |
12 |         Err("No elements".to_owned());
   |                                      ^
help: consider using a semicolon here
   |
13 |     };
   |      ^
help: you might have meant to return this value
   |
12 |         return Err("No elements".to_owned());
   |         ^^^^^^                              ^
