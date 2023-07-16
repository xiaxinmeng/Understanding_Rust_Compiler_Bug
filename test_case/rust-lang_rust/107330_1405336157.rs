rust
   Compiling rustc_panic v0.1.0 (/tmp/tmp.kV9ctrxHyG/rustc_panic)
error[E0308]: mismatched types
  --> src/post.rs:11:9
   |
9  |   pub trait Post<'p>: Iterator<Item = Event<'p>> {
   |   ---------------------------------------------- this type parameter
10 |       fn skip_closing(&mut self) -> Filter<Self, _> {
   |                                     --------------- expected `Filter<Self, _>` because of return type
11 | /         self.filter(|event| match event {
12 | |             Event::End(_) => false,
13 | |             _ => true,
14 | |         })
   | |__________^ expected type parameter `Self`, found `&mut Self`
   |
   = note: expected struct `Filter<Self, _>`
              found struct `Filter<&mut Self, [closure@src/post.rs:11:21: 11:28]>`

error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
  --> src/post.rs:10:48
   |
10 |     fn skip_closing(&mut self) -> Filter<Self, _> {
   |                                                ^ not allowed in type signatures

Some errors have detailed explanations: E0121, E0308.
For more information about an error, try `rustc --explain E0121`.
error: could not compile `rustc_panic` due to 2 previous errors
