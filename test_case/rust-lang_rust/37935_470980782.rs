
error[E0308]: match arms have incompatible types
  --> src/main.rs:16:22
   |
13 | /             match x {
14 | |                 Foo::A => true,
   | |                           ---- this is found to be of type `bool`
15 | |                 Foo::B => false,
   | |                           ----- this is found to be of type `bool`
16 | |                 _ => unreachable!()
   | |                      ^^^^^^^^^^^^^^ expected bool, found ()
17 | |             }
   | |_____________- `match` arms have incompatible types
   |
   = note: expected type `bool`
              found type `()`
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
