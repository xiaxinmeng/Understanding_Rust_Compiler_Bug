
error[E0277]: the trait bound `[u8; N]: std::default::Default` is not satisfied
  --> /home/lcnr/rust7/src/test/ui/__check/fk.rs:7:13
   |
LL |         Foo(Default::default())
   |             ^^^^^^^^^^^^^^^^^^ the trait `std::default::Default` is not implemented for `[u8; N]`
   |
   = help: the following implementations were found:
             <&[T] as std::default::Default>
             <&mut [T] as std::default::Default>
             <[T; 0] as std::default::Default>
             <[T; 10] as std::default::Default>
           and 32 others
   = note: required by `std::default::Default::default`

error: aborting due to previous error; 1 warning emitted
