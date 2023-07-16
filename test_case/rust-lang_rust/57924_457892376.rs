
error[E0423]: expected function, found self type `Self`
  --> <source>:18:9
   |
18 |         Self::<E>(Processor::<E>::new(key))
   |         ^^^^^^^^^ not a function
   |
   = note: can't use `Self` as a constructor, you must use the implemented struct

error: aborting due to previous error

For more information about this error, try `rustc --explain E0423`.
