
error[E0597]: `foo2` does not live long enough
  --> src/lib.rs:12:25
   |
12 |         self.iter().zip(foo2.iter()).all(|(&x, &y)| x > y)
   |         ----------------^^^^--------
   |         |               |
   |         |               borrowed value does not live long enough
   |         a temporary with access to the borrow is created here ...
...
15 |     }
   |     -
   |     |
   |     `foo2` dropped here while still borrowed
   |     ... and the borrow might be used here, when that temporary is dropped and runs the
destructor for type `std::iter::Zip<impl std::iter::Iterator, impl std::iter::Iterator>`
   |
   = note: The temporary is part of an expression at the end of a block. Consider forcing this
temporary to be dropped sooner, before the block's local variables are dropped. For example,
you could save the expression's value in a new local variable `x` and then make `x` be the
expression at the end of the block.
