
error[E0601]: `main` function not found in crate `hang`
  --> hang.rs:47:2
   |
47 | }
   |  ^ consider adding a `main` function to `hang.rs`

error[E0275]: overflow evaluating the requirement `<Filter<S, P> as LendingIterator>::Item<'a>`
  --> hang.rs:29:1
   |
29 | impl<S, P> LendingIterator for Filter<S, P>
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`hang`)

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0275, E0601.
For more information about an error, try `rustc --explain E0275`.
