
error[E0271]: type mismatch resolving `<impl Future as Future>::Output == impl Stream`
  --> src/main.rs:22:43
   |
20 |     type LineStream<'a, Repr> = impl Stream<Item = Repr>;
   |                                 ------------------------ the expected opaque type
21 |     type LineStreamFut<'a, Repr> = impl Future<Output = Self::LineStream<'a, Repr>>;
22 |     fn line_stream<'a, Repr>(&'a self) -> Self::LineStreamFut<'a, Repr> {
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found struct `Empty`
   |
   = note: expected opaque type `impl Stream`
                   found struct `Empty<_>`
