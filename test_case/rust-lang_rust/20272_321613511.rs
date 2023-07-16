
error: expected one of `(`, `,`, `?`, `for`, `{`, lifetime, or path, found `ATrait`
 --> src/main.rs:4:55
  |
4 |         impl<'a, T> Somethingable for $Block where T: $Base {
  |                                                       ^^^^^ expected one of 7 possible tokens here
...
9 | impl_somethingable!(ATrait, A);
  | ------------------------------- in this macro invocation
