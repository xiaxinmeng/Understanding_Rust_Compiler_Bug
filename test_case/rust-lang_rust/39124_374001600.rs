
error[E0308]: mismatched types
  --> a.rs:27:9
   |
26 |     fn deserialize_bytes<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error> {
   |                                                           ----------------------------- expected `std::result::Result<<V as Visitor>::Value, ()>` because of return type
27 |         visitor.visit_bytes(self)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
   |
   = note: expected type `std::result::Result<_, ()>`
              found type `std::result::Result<_, <V as Visitor>::Error>`

error: aborting due to previous error
