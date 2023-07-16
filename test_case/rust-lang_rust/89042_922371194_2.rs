
error[E0004]: non-exhaustive patterns: `B` not covered
  --> /home/devinr/aprog/rust/__forks__/rust/src/test/ui/rfc-2008-non-exhaustive/reachable-unstable.rs:43:11
   |
LL | / enum Foo {
LL | |     A,
LL | |     B,
   | |     - not covered
LL | |     #[doc(hidden)]
LL | |     C,
LL | | }
   | |_- `Foo` defined here
...
LL |       match Foo::A {
   |             ^^^^^^ pattern `B` not covered
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `Foo`
