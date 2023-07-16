
error[E0603]: module `mem` is private
 --> src/main.rs:2:9
  |
2 |     use foo::mem;
  |         ^^^^^^^^
...
6 |     use std::mem;
  |     ^^^^^^^^^^^^^ private module
  |     |
  |     help: consider making it public: `pub use std::mem;`
