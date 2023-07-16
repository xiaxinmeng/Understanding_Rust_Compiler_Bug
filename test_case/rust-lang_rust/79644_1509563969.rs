
error[[E0277]](https://doc.rust-lang.org/nightly/error_codes/E0277.html): expected a `Fn<(&<S as Service>::Output,)>` closure, found `M`
  --> src/main.rs:71:20
   |
71 |     type Promise = MagicPromise<S::Promise, M>;
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `Fn<(&<S as Service>::Output,)>` closure, found `M`
   |
note: required for `M` to implement `Magician<<S as Service>::Output>`
  --> src/main.rs:10:12
   |
10 | impl<F, T> Magician<T> for F
   |            ^^^^^^^^^^^     ^
11 | where
12 |     F: Fn(&T),
   |        ------ unsatisfied trait bound introduced here
note: required for `MagicPromise<<S as Service>::Promise, M>` to implement `Promise`
  --> src/main.rs:42:12
   |
42 | impl<P, M> Promise for MagicPromise<P, M>
   |            ^^^^^^^     ^^^^^^^^^^^^^^^^^^
...
45 |     M: Magician<P::Output>,
   |        ------------------- unsatisfied trait bound introduced here
note: required by a bound in `Service::Promise`
  --> src/main.rs:29:27
   |
29 |     type Promise: Promise<Output = Self::Output>;
   |                           ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Service::Promise`
help: consider further restricting this bound
   |
66 |     M: Magician<()> + Clone + 'static + for<'a> std::ops::Fn<(&'a <S as Service>::Output,)>,
   |                                       +++++++++++++++++++++++++++++++++++++++++++++++++++++
 