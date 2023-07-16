
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
  --> src/main.rs:30:13
   |
29 |         if let Err(err) = create(&self.owned) {
   |                           -------------------
   |                           |      |
   |                           |      immutable borrow occurs here
   |                           a temporary with access to the immutable borrow is created here ...
30 |             self.on_error(err)
   |             ^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
31 |         }
32 |     }
   |     - ... and the immutable borrow might be used here, when that temporary is dropped and runs the destructor for type `std::result::Result<HoldsReference<'_>, Error>`
   |
help: consider adding semicolon after the expression so its temporaries are dropped sooner, before the local variables declared by the block are dropped
   |
31 |         };
   |          ^
