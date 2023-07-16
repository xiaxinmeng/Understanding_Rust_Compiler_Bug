log
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> src/lib.rs:10:23
   |
10 |           thread::spawn(move || {
   |  _______________________^
11 | |             println!("something {}",&self.a);
12 | |         });
   | |_________^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 9:5...
  --> src/lib.rs:9:5
   |
9  | /     pub fn foo(&self) {
10 | |         thread::spawn(move || {
11 | |             println!("something {}",&self.a);
12 | |         });
13 | |     }
   | |_____^
   = note: ...so that the types are compatible:
           expected &A
              found &A
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that the type `[closure@src/lib.rs:10:23: 12:10 self:&A]` will meet its required lifetime bounds
  --> src/lib.rs:10:9
   |
10 |         thread::spawn(move || {
   |         ^^^^^^^^^^^^^
