
error[E0599]: the method `check` exists for struct `Client<()>`, but its trait bounds were not satisfied
  --> f9.rs:81:16
   |
49 | struct ALayer<C>(C);
   | ---------------- doesn't satisfy `ALayer<()>: ParticularServiceLayer<()>`
...
63 | struct Client<C>(C);
   | ---------------- method `check` not found for this struct
...
81 |     Client(()).check(); //~ ERROR E0599
   |                ^^^^^ method cannot be called on `Client<()>` due to unsatisfied trait bounds
   |
note: trait bound `ALayer<()>: ParticularServiceLayer<()>` was not satisfied
  --> f9.rs:69:16
   |
67 | impl<C> Client<C>
   |         ---------
68 | where
69 |     ALayer<C>: ParticularServiceLayer<C>,
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound introduced here
note: the trait `ParticularServiceLayer` must be implemented
  --> f9.rs:34:1
   |
34 | pub trait ParticularServiceLayer<C>: Layer<C, Service = <Self as ParticularServiceLayer<C>>::Service> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
