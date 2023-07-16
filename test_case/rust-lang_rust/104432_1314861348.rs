`
warning: lifetime parameter `'s` never used
 --> src/test/ui/deriving/issue-89188-gat-hrtb.rs:6:62
  |
5 | #[derive(Clone)]
  |          - help: elide the unused lifetime
6 | struct ShimMethod4<T: Trait2 + 'static>(pub &'static dyn for<'s> Fn(&'s mut T::As));
  |                                                              ^^
  |
  = note: requested on the command line with `-W unused-lifetimes`

warning: 1 warning emitted
