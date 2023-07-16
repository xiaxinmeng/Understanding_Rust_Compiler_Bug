plain
   Compiling quickcheck v0.9.2
   Compiling ipnetwork v0.17.0
   Compiling diesel_derives v2.0.0 (/checkout/obj/build/ct/diesel/diesel_derives)
   Compiling diesel v2.0.0 (/checkout/obj/build/ct/diesel/diesel)
error[E0275]: overflow evaluating the requirement `&(_, _, _, _, _): insertable::Insertable<_>`
  |
  = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`diesel`)
  = note: required because of the requirements on the impl of `insertable::Insertable<_>` for `(&(_, _, _, _, _), &_, &_, &_, &_)`
  = note: 127 redundant requirements hidden
  = note: required because of the requirements on the impl of `insertable::Insertable<_>` for `&(((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((_, _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _), _, _, _, _)`
For more information about this error, try `rustc --explain E0275`.
error: could not compile `diesel` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
---
expected success, got: exit status: 101


Build completed unsuccessfully in 0:26:04
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
