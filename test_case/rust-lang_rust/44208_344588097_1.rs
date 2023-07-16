
error[E0034]: multiple applicable items in scope
  --> src/main.rs:35:34
   |
35 |     let _ = ARR.make_container().do_something();
   |                                  ^^^^^^^^^^^^ multiple `do_something` found
   |
note: candidate #1 is defined in an impl for the type `Container<&'static _>`
  --> src/main.rs:6:5
   |
6  | /     fn do_something(&self) {
7  | |         println!("Container<&'a T>::do_something");
8  | |     }
   | |_____^
note: candidate #2 is defined in an impl for the type `Container<&mut _>`
  --> src/main.rs:12:5
   |
12 | /     fn do_something(&self) {
13 | |         println!("Container<&'a mut T>::do_something");
14 | |     }
   | |_____^
