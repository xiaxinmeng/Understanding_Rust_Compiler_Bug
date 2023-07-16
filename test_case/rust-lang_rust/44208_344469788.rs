
   Compiling playground v0.0.1 (file:///playground)
error[E0034]: multiple applicable items in scope
  --> src/main.rs:21:43
   |
21 |     let x: Option<char> = TABLE.find('a').cloned();
   |                                           ^^^^^^ multiple `cloned` found
   |
   = note: candidate #1 is defined in an impl for the type `std::option::Option<&_>`
   = note: candidate #2 is defined in an impl for the type `std::option::Option<&mut _>`

error: aborting due to previous error

error: Could not compile `playground`.
