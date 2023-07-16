
error[E0277]: the trait bound `C: B<{&_: &'static A}>` is not satisfied
  --> src/lib.rs:21:23
   |
21 |     B::<{ &A::A }>::a(&C);
   |     ----------------- ^^ the trait `B<{&_: &'static A}>` is not implemented for `C`
   |     |
   |     required by a bound introduced by this call
   |
   = help: the following implementations were found:
             <C as B<{&_: &'static A}>>
             <C as B<{&_: &'static A}>>
note: required by `B::a`
  --> src/lib.rs:11:5
   |
11 |     fn a(&self) {}
   |     ^^^^^^^^^^^
