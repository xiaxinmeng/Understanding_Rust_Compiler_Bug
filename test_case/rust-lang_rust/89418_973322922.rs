
error[E0599]: the method `mini_load_payload` exists for struct `MiniStructProvider<_>`, but its trait bounds were not satisfied
  --> f22.rs:65:49
   |
30 | / struct MiniStructProvider<M>
31 | | where
32 | |     M: MiniDataMarker,
33 | | {
34 | |     pub yoke: MiniYoke<M::Yokeable>,
35 | | }
   | | -
   | | |
   | |_method `mini_load_payload` not found for this
   |   doesn't satisfy `MiniStructProvider<_>: MiniDataProvider<_>`
...
65 |       let yoke: MiniYoke<SimpleStruct> = provider.mini_load_payload();
   |                                                   ^^^^^^^^^^^^^^^^^ method cannot be called on `MiniStructProvider<_>` due to unsatisfied trait bounds
   |
note: the following trait bounds were not satisfied because of the requirements of the implementation of `MiniDataProvider<_>` for `_`:
      `<_ as MiniYokeable<'a>>::Output: Clone`
  --> f22.rs:40:56
   |
37 | impl<M> MiniDataProvider<M> for MiniStructProvider<M>
   |         -------------------     --------------------- ...for this type
   |         |
   |         in the implementation of this trait...
...
40 |     for<'a> <M::Yokeable as MiniYokeable<'a>>::Output: Clone,
   |                                                        ^^^^^ obligation introduced here
