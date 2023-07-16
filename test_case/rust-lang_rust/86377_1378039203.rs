
error[E0599]: the method `check` exists for struct `Client<()>`, but its trait bounds were not satisfied
  --> src/lib.rs:82:16
   |
49 | struct ALayer<C>(C);
   | ---------------- doesn't satisfy `ALayer<()>: ParticularServiceLayer<()>`
...
63 | struct Client<C>(C);
   | ---------------- method `check` not found for this struct
...
82 |     Client(()).check();
   |                ^^^^^ method cannot be called on `Client<()>` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `ALayer<()>: ParticularServiceLayer<()>`
note: the trait `ParticularServiceLayer` must be implemented
  --> src/lib.rs:34:1
   |
34 | pub trait ParticularServiceLayer<C>: Layer<C, Service = <Self as ParticularServiceLayer<C>>::Service> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0271]: type mismatch resolving `<AService as Service<Req>>::Response == Res`
  --> src/lib.rs:86:11
   |
86 |     check(());
   |     ----- ^^ type mismatch resolving `<AService as Service<Req>>::Response == Res`
   |     |
   |     required by a bound introduced by this call
   |
note: expected this to be `Res`
  --> src/lib.rs:58:21
   |
58 |     type Response = bool;
   |                     ^^^^
note: required for `AService` to implement `ParticularService`
  --> src/lib.rs:22:9
   |
22 | impl<T> ParticularService for T
   |         ^^^^^^^^^^^^^^^^^     ^
note: required for `ALayer<_>` to implement `ParticularServiceLayer<_>`
  --> src/lib.rs:38:12
   |
38 | impl<T, C> ParticularServiceLayer<C> for T
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^     ^
note: required by a bound in `check`
  --> src/lib.rs:74:36
   |
74 | fn check<C>(_: C) where ALayer<C>: ParticularServiceLayer<C> {}
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `check`
