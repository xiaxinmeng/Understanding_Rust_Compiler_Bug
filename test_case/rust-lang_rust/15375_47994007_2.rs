 rust
let discrim_self = ...;
let discrim_arg_0 = ...;
match (*self, *__arg_0) {
  (A(ref __self_0), A(ref __arg_1_0)) => /* recur */,
  (B(ref __self_0), B(ref __arg_1_0)) => /* recur */,
  (C(ref __self_0), C(ref __arg_1_0)) => /* recur */,
  // N variants, but each has code that is bounded in complexity by the particular variant
  // thus we should be O(N) overall.
  ...
  (_, _) => discrim_self.cmp(discrim_arg_0)
}
