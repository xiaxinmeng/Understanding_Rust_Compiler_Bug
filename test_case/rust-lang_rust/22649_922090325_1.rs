text
error[E0529]: expected an array or slice, found `Vec<&str>`
  --> src/main.rs:13:9
   |
13 |         ["some", sub] => println!("some {}", sub),
   |         ^^^^^^^^^^^^^ pattern cannot match with input type `Vec<&str>`

error[E0529]: expected an array or slice, found `Vec<&str>`
  --> src/main.rs:14:9
   |
14 |         ["quit"] => println!("bye!"),
   |         ^^^^^^^^ pattern cannot match with input type `Vec<&str>`

For more information about this error, try `rustc --explain E0529`.
error: could not compile `playground` due to 2 previous errors
