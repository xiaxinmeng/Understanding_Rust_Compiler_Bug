
error[E0271]: type mismatch resolving `<AService as Service<Req>>::Response == Res`
  --> f9.rs:85:11
   |
85 |     check(()); //~ ERROR E0271
   |     ----- ^^ type mismatch resolving `<AService as Service<Req>>::Response == Res`
   |     |
   |     required by a bound introduced by this call
   |
note: expected this to be `Res`
  --> f9.rs:58:21
   |
58 |     type Response = bool;
   |                     ^^^^
note: required for `AService` to implement `ParticularService`
  --> f9.rs:22:9
   |
22 | impl<T> ParticularService for T
   |         ^^^^^^^^^^^^^^^^^     ^
23 | where
24 |     T: Service<Req, Response = Res>,
   |                     -------------- unsatisfied trait bound introduced here
note: required for `ALayer<_>` to implement `ParticularServiceLayer<_>`
  --> f9.rs:38:12
   |
38 | impl<T, C> ParticularServiceLayer<C> for T
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^     ^
...
41 |     T::Service: ParticularService,
   |                 ----------------- unsatisfied trait bound introduced here
note: required by a bound in `check`
  --> f9.rs:74:36
   |
74 | fn check<C>(_: C) where ALayer<C>: ParticularServiceLayer<C> {}
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `check`
