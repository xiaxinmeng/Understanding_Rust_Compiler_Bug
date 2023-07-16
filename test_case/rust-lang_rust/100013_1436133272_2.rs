
error[E0478]: lifetime bound not satisfied
  --> src/lib.rs:10:1
   |
10 | / impl<T: Tr> S<T>
11 | | where
12 | |     for<'a, 'b> T: Tr<A<'a, 'b> = ()>,
13 | |     for<'c, 'd> T::A<'c, 'd>: Default,
14 | | {
15 | | }
   | |_^
