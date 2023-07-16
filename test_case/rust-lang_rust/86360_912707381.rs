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
For more information about this error, try `rustc --explain E0275`.
error: could not compile `diesel` due to previous error
error: build failed
thread 'main' panicked at 'tests failed for https://github.com/diesel-rs/diesel', src/tools/cargotest/main.rs:101:9



note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/cargo" "/checkout/obj/build/ct"
expected success, got: exit status: 101


Build completed unsuccessfully in 0:18:59
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
