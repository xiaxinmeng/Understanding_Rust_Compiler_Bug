
error[E0275]: overflow evaluating the requirement `UInt<(), ()>: PrivatePow<UInt<(), ()>, UInt<_, _>>`
  --> src/main.rs:30:5
   |
30 |     Tensor::<(), ()>::new
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate
   = note: required because of the requirements on the impl of `PrivatePow<UInt<(), ()>, UInt<_, _>>` for `UInt<(), ()>`
   = note: required because of the requirements on the impl of `PrivatePow<UInt<(), ()>, UInt<_, _>>` for `UInt<(), ()>`
[the line above repeated a thousand more times]
