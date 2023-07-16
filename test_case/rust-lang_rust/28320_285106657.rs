powershell
C:\Users\steve\tmp> vim c.rs
C:\Users\steve\tmp> rustc .\c.rs --crate-type=rlib
warning: method is never used: `V`, #[warn(dead_code)] on by default
  --> .\c.rs:6:5
   |
6  |       fn V(u: u8) -> E {
   |  _____^ starting here...
7  | |         println!("IN V!");
8  | |
9  | |         E::V(u)
10 | |     }
   | |_____^ ...ending here

warning: method `V` should have a snake case name such as `v`, #[warn(non_snake_case)] on by default
  --> .\c.rs:6:5
   |
6  |       fn V(u: u8) -> E {
   |  _____^ starting here...
7  | |         println!("IN V!");
8  | |
9  | |         E::V(u)
10 | |     }
   | |_____^ ...ending here

C:\Users\steve\tmp> vim d.rs
C:\Users\steve\tmp> rustc .\d.rs -L .
warning: function is never used: `test`, #[warn(dead_code)] on by default
 --> .\d.rs:4:1
  |
4 |   fn test(e: &E) {
  |  _^ starting here...
5 | |     match *e {
6 | |         E::V(_) => println!("Yep"),
7 | |     }
8 | | }
  | |_^ ...ending here

C:\Users\steve\tmp> .\d.exe
123
