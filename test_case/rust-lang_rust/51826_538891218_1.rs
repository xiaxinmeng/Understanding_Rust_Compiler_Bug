
error[E0502]: cannot borrow `self.0` as mutable because it is also borrowed as immutable
  --> src/main.rs:15:13
   |
14 |         if let Some(x) = self.iter2().find(|_| true) {
   |                          ------------
   |                          |
   |                          immutable borrow occurs here
   |                          a temporary with access to the immutable borrow is created here ...
15 |             self.0.push(x);
   |             ^^^^^^ mutable borrow occurs here
16 |         }
17 |     }
   |     - ... and the immutable borrow might be used here, when that temporary is dropped and runs the destructor for type `impl std::iter::Iterator`
   |
   = note: The temporary is part of an expression at the end of a block. Consider adding semicolon after the expression so its temporaries are dropped sooner, before the local variables declared by the block are dropped.
