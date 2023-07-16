
error[E0308]: mismatched types
  --> src/lib.rs:10:39
   |
10 |     const C: <Self::Fv as Foo>::Bar = 6665;
   |                                       ^^^^ expected associated type, found integer
   |
   = note: expected type `<<Self as Baz>::Fv as Foo>::Bar`
              found type `{integer}`
