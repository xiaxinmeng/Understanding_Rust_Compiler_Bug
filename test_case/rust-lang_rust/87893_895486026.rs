
error[E0283]: type annotations needed
 --> src/lib.rs:8:40
  |
1 | trait TraitWithLifetime<'a> {
  | --------------------------- required by this bound in `TraitWithLifetime`
...
8 |     T: for<'b> TraitWithLifetime<'b> + TraitWithLifetime<'a>
  |                                        ^^^^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `T`
  |
  = note: cannot satisfy `T: TraitWithLifetime<'a>`
