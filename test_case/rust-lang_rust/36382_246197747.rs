
vladimir@grizzly[2334]$ cargo build
   Compiling ipc-channel v0.5.1 (file:///C:/proj/r/ipc-channel)
error[E0308]: mismatched types
  --> src\platform/mod.rs:18:5
   |
18 |     include!("windows/mod.rs");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected &str, found struct `std::string::String`
   |
   = note: expected type `&str`
   = note:    found type `std::string::String`
   = note: this error originates in a macro from the standard library

error: aborting due to previous error

error: Could not compile `ipc-channel`.
