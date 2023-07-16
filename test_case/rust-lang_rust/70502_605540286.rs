
error[E0308]: mismatched types
  --> /home/jistone/rust/rust/src/test/ui/variance-iterators-in-libcore.rs:7:67
   |
LL | fn fuse_covariant<'a, I>(iter: Fuse<&'static I>) -> Fuse<&'a I> { iter }
   |                                                                   ^^^^ lifetime mismatch
   |
   = note: expected struct `std::iter::Fuse<&'a I>`
              found struct `std::iter::Fuse<&'static I>`
note: the lifetime `'a` as defined on the function body at 7:19...
  --> /home/jistone/rust/rust/src/test/ui/variance-iterators-in-libcore.rs:7:19
   |
LL | fn fuse_covariant<'a, I>(iter: Fuse<&'static I>) -> Fuse<&'a I> { iter }
   |                   ^^
   = note: ...does not necessarily outlive the static lifetime
