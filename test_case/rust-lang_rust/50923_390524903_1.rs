
   Compiling playground v0.0.1 (file:///playground)
error[E0412]: cannot find type `Input` in this scope
  --> src/main.rs:21:15
   |
21 |     fn foo(x: Input) {  }
   |               ^^^^^ not found in this scope
help: possible candidates are found in other modules, you can import them into scope
   |
21 |     use a::Input;
   |
21 |     use b::Input;
   |
21 |     use c::Input;
   |
