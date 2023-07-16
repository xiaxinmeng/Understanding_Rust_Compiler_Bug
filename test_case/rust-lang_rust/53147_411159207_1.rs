none
error[E0507]: cannot move out of borrowed content
  --> a.rs:13:23
   |
13 |     let &X { a, b } = &x;
   |              -  -     ^^ cannot move out of borrowed content
   |              |  |
   |              |  ... and here
   |              data moved here
   |
note: move occurs because a has type `std::string::String`, which does not implement the `Copy` trait
  --> a.rs:13:14
   |
13 |     let &X { a, b } = &x;
   |              ^
note: move occurs because b has type `std::string::String`, which does not implement the `Copy` trait
  --> a.rs:13:17
   |
13 |     let &X { a, b } = &x;
   |                 ^
help: consider removing this borrow operator
   |
13 |     let X { a, b } = &x;
   |         ^^^^^^^^^^
help: consider removing this borrow operator
   |
13 |     let X { a, b } = &x;
   |         ^^^^^^^^^^

