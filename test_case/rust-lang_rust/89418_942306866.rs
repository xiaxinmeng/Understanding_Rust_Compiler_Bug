
error[E0599]: the method `mini_load_payload` exists for struct `MiniStructProvider<_>`, but its trait bounds were not satisfied
  --> f16.rs:85:59
   |
47 | / struct MiniStructProvider<M>
48 | | where
49 | |     M: MiniDataMarker,
50 | | {
51 | |     pub payload: MiniDataPayload<M>,
52 | | }
   | | -
   | | |
   | |_method `mini_load_payload` not found for this
   |   doesn't satisfy `MiniStructProvider<_>: MiniDataProvider<_>`
...
85 |       let payload: MiniDataPayload<SimpleStruct> = provider.mini_load_payload();
   |                                                             ^^^^^^^^^^^^^^^^^ method cannot be called on `MiniStructProvider<_>` due to unsatisfied trait bounds
   |
note: the following trait bounds were not satisfied because of the requirements of the implementation of `MiniDataProvider<_>` for `_`:
      `<_ as MiniYokeable<'a>>::Output: Clone`
  --> f16.rs:54:9
   |
54 | impl<M> MiniDataProvider<M> for MiniStructProvider<M>
   |         ^^^^^^^^^^^^^^^^^^^     ^^^^^^^^^^^^^^^^^^^^^
