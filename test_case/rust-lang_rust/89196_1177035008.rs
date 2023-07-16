
error[E0599]: the method `mini_load_payload` exists for struct `MiniStructProvider<_>`, but its trait bounds were not satisfied
  --> src/main.rs:65:49
   |
30 | struct MiniStructProvider<M>
   | ----------------------------
   | |      |
   | |      method `mini_load_payload` not found for this struct
   | doesn't satisfy `MiniStructProvider<_>: MiniDataProvider<_>`
...
65 |     let yoke: MiniYoke<SimpleStruct> = provider.mini_load_payload();
   |                                                 ^^^^^^^^^^^^^^^^^ method cannot be called on `MiniStructProvider<_>` due to unsatisfied trait bounds
   |
note: trait bound `<_ as MiniYokeable<'a>>::Output: Clone` was not satisfied
  --> src/main.rs:40:56
   |
37 | impl<M> MiniDataProvider<M> for MiniStructProvider<M>
   |         -------------------     ---------------------
...
40 |     for<'a> <M::Yokeable as MiniYokeable<'a>>::Output: Clone,
   |                                                        ^^^^^ unsatisfied trait bound introduced here
