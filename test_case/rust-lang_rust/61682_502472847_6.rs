rust
[...]

error[E0308]: mismatched types
  --> src/lib.rs:14:17
   |
8  |     fn map<B>(self, arrow: impl FnOnce(A) -> B) -> Option<B> {
   |                                                    --------- expected `Option<B>` because of return type
...
14 |                 Self::None
   |                 ^^^^^^^^^^ expected type parameter, found a different type parameter
   |
   = note: expected type `Option<B>`
              found type `Option<A>`
