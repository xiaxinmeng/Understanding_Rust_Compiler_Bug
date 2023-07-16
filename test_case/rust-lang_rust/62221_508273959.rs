
error[E0308]: mismatched types
  --> /home/jonas/dev/rust/src/test/ui/impl-trait/bound-normalization-pass.rs:46:56
   |
LL |     fn foo2_pass<'a, T: Trait<'a, Assoc=()> + 'a>() -> impl FooLike<Output=T::Assoc> + 'a {
   |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected type `lifetimes::Trait<'a>`
              found type `lifetimes::Trait<'static>`
note: the lifetime 'a as defined on the function body at 46:18...
  --> /home/jonas/dev/rust/src/test/ui/impl-trait/bound-normalization-pass.rs:46:18
   |
LL |     fn foo2_pass<'a, T: Trait<'a, Assoc=()> + 'a>() -> impl FooLike<Output=T::Assoc> + 'a {
   |                  ^^
   = note: ...does not necessarily outlive the static lifetime
