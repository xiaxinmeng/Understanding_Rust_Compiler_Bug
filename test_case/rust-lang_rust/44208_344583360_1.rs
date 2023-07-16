
error[E0034]: multiple applicable items in scope
  --> src/main.rs:19:43
   |
19 |     let x: Option<char> = TABLE.find('a').cloned();
   |                                           ^^^^^^ multiple `cloned` found
   |
   = note: candidate #1 is defined in an impl for the type `std::option::Option<&_>`
   = note: candidate #2 is defined in an impl for the type `std::option::Option<&mut _>`
