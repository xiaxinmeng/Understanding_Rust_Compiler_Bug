
error[E0311]: the parameter type `Self` may not live long enough
   |
   = help: consider adding an explicit lifetime bound `Self: 'a`...
   = note: ...so that the type `Self` will meet its required lifetime bounds...
note: ...that is required by this bound
  --> <source>:12:24
   |
12 | pub trait B: for<'a> A<Z<'a> = Iter<'a, <Self as A>::Y<'a>>> {}
   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
