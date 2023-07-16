rust
error[E0275]: overflow evaluating the requirement `[closure@erlust/src/lib.rs:10:34: 10:40]: std::marker::Freeze`
  |
  = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
  = note: required because of the requirements on the impl of `std::marker::Freeze` for `[closure@erlust/src/lib.rs:10:34: 10:40]`
[…]
  = note: required because of the requirements on the impl of `std::marker::Freeze` for `[closure@erlust/src/lib.rs:10:34: 10:40]`
  = note: required because it appears within the type `std::option::Option<[closure@erlust/src/lib.rs:10:34: 10:40]>`
  = note: required because it appears within the type `futures_util::future::lazy::Lazy<[closure@erlust/src/lib.rs:10:34: 10:40]>`
  = note: required because it appears within the type `{futures_util::future::lazy::Lazy<[closure@erlust/src/lib.rs:10:34: 10:40]>, ()}`
  = note: required because it appears within the type `[static generator@erlust/src/lib.rs:9:1: 11:2 {futures_util::future::lazy::Lazy<[closure@erlust/src/lib.rs:10:34: 10:40]>, ()}]`
  = note: required because it appears within the type `std::future::GenFuture<[static generator@erlust/src/lib.rs:9:1: 11:2 {futures_util::future::lazy::Lazy<[closure@erlust/src/lib.rs:10:34: 10:40]>, ()}]>`
  = note: required because it appears within the type `impl core::future::future::Future`
  = note: required because it appears within the type `impl core::future::future::Future`
  = note: required because it appears within the type `for<'r> {impl core::future::future::Future, ()}`
  = note: required because it appears within the type `[static generator@erlust/src/lib.rs:14:11: 14:50 for<'r> {impl core::future::future::Future, ()}]`
  = note: required because it appears within the type `std::future::GenFuture<[static generator@erlust/src/lib.rs:14:11: 14:50 for<'r> {impl core::future::future::Future, ()}]>`
  = note: required because it appears within the type `impl core::future::future::Future`

error: aborting due to previous error
