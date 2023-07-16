rust
error[E0277]: the trait bound `Foo<'_>: beef::traits::internal::Beef` is not satisfied
  --> src/main.rs:43:20
   |
43 |     dbg!(size_of::<beef::Cow<'_, Foo>>());
   |                    ^^^^^^^^^^^^^^^^^^ the trait `beef::traits::internal::Beef` is not implemented for `Foo<'_>`
   |
  ::: /home/lzutao/.cargo/registry/src/github.com-1ecc6299db9ec823/beef-0.4.4/src/generic.rs:22:23
   |
22 | pub struct Cow<'a, T: Beef + ?Sized + 'a, U: Capacity> {
   |                       ---- required by this bound in `beef::generic::Cow`
