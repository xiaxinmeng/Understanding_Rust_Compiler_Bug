
error[E0499]: cannot borrow `thing` as mutable more than once at a time
  --> src/main.rs:32:22
   |
32 |         let result = thing.process(); 
   |                      ^^^^^----------
   |                      |
   |                      mutable borrow starts here in previous iteration of loop
   |                      argument requires that `thing` is borrowed for `'static`
