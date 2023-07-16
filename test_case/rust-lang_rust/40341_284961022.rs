
error[E0412]: cannot find type `File` in this scope
 --> example.rs:4:25
  |
4 |     fn some_function(f: File) {}
  |                         ^^^^ not found in this scope
  |
  = help: possible candidate is found in another module, you can import it into scope:
            `use std::fs::File;`
