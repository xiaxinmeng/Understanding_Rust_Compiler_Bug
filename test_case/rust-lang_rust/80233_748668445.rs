
error[E0277]: the trait bound `usize: Trait3` is not satisfied
  --> fulfill.rs:10:14
   |
10 |     pub ins: <<usize as Trait3>::Type3 as Trait2>::Type2,
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Trait3` is not implemented for `usize`
