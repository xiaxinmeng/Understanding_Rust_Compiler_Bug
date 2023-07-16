rust
5917   │         // Parse optional colon and supertrait bounds.
5918   │         let bounds = if self.eat(&token::Colon) {
5919   │             self.parse_generic_bounds()?
 