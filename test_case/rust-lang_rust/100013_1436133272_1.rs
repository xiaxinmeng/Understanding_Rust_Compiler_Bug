
error: lifetime bound not satisfied
  --> src/lib.rs:10:1
   |
10 | / impl<T: Tr> S<T>
11 | | where
12 | |     for<'a, 'b> T: Tr<A<'a, 'b> = ()>,
13 | |     for<'c, 'd> T::A<'c, 'd>: Default,
14 | | {
15 | | }
   | |_^
   |
note: the lifetime `'c` defined here...
  --> src/lib.rs:13:6
   |
13 |     for<'c, 'd> T::A<'c, 'd>: Default,
   |         ^^
note: ...must outlive the lifetime `'d` defined here
  --> src/lib.rs:13:10
   |
13 |     for<'c, 'd> T::A<'c, 'd>: Default,
   |             ^^
   = note: this is a known limitation that will be removed in the future (see issue #100013 <https://github.com/rust-lang/rust/issues/100013> for more information)
