
   Compiling playground v0.0.1 (file:///playground)
error[E0275]: overflow evaluating the requirement `<<<<<<T as ListString>::Tail as ListString>::Tail as ListString>::Tail as ListString>::Tail as ListString>::Tail as ListString>::Tail: ListString`
  |
  = help: consider adding a `#![recursion_limit="24"]` attribute to your crate
  = note: required because it appears within the type `Cons<impl ListString>`
  = note: required because it appears within the type `impl ListString`
  = note: required because it appears within the type `Cons<impl ListString>`
  = note: required because it appears within the type `impl ListString`
  = note: required because it appears within the type `Cons<impl ListString>`
  = note: required because it appears within the type `impl ListString`
  = note: required because it appears within the type `Cons<impl ListString>`
  = note: required because it appears within the type `impl ListString`
  = note: required because it appears within the type `Cons<impl ListString>`
  = note: required because it appears within the type `impl ListString`
  = note: required because it appears within the type `Cons<impl ListString>`
  = note: required because it appears within the type `impl ListString`

error: aborting due to previous error

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
