
error: cannot find macro `printlx!` in this scope
  --> $DIR/macro-name-typo.rs:12:5
   |
12 |     printlx!("oh noes!"); //~ ERROR cannot find
   |     ^^^^^^^ help: you could try the macro: `println!`
   = help: although `printlx` wasn't found, there's a macro named `println`:
   |
12 |     println!("oh noes!"); //~ ERROR cannot find
   |     ^^^^^^^

