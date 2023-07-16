
error[E0277]: the trait bound `(S, S): NotSame` is not satisfied
  --> src/main.rs:14:5
   |
7  | fn f<T, U>(_: T, _: U) where (T, U): NotSame {}
   |                                      ------- required by this bound in `f`
...
14 |     f::<S, S>(S, S); // error[E0277]: the trait bound `(S, S): NotSame` is not satisfied
   |     ^^^^^^^^^ the trait `NotSame` is not implemented for `(S, S)`
   |
   = help: the following implementations were found:
             <(A, A) as NotSame>

error[E0277]: the trait bound `(&'a (), &'static ()): NotSame` is not satisfied
  --> src/main.rs:18:5
   |
7  | fn f<T, U>(_: T, _: U) where (T, U): NotSame {}
   |                                      ------- required by this bound in `f`
...
18 |     f::<&'a (), &'static ()>(&(), &()); 
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `NotSame` is not implemented for `(&'a (), &'static ())`
   |
   = help: the following implementations were found:
             <(A, A) as NotSame>
