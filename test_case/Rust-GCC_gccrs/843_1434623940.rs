rust
error[E0223]: ambiguous associated type
  --> test.rs:15:10
   |
15 | type A = <S as Tr>::A::f<u8>;
   |          ^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<<S as Tr>::A as Trait>::f`

error: aborting due to previous error
