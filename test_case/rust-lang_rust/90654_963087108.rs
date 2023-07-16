
error[E0277]: the trait bound `[i32; _]: SmallArray` is not satisfied
  --> ../unknown-vars-resolve-instance/test.rs:22:13
   |
22 |     consume(make_array::<T>());
   |     ------- ^^^^^^^^^^^^^^^^^ the trait `SmallArray` is not implemented for `[i32; _]`
   |     |
   |     required by a bound introduced by this call
   |
note: required by a bound in `consume`
  --> ../unknown-vars-resolve-instance/test.rs:15:15
   |
15 | fn consume<T: SmallArray>(t: T) {}
   |               ^^^^^^^^^^ required by this bound in `consume`
help: consider extending the `where` bound, but there might be an alternative better way to express this requirement
   |
20 |     [(); T::VAL + 1]:, [i32; _]: SmallArray
   |                      ~~~~~~~~~~~~~~~~~~~~~~
