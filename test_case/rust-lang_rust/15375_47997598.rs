 rust
let discrim_self = match *self { A(..) => 0u, B(..) => 1u, C(..) => 2u, ... }; // N variants
let discrim_arg_0 = match *__arg_0 { A(..) => 0u, B(..) => 1u, C(..) => 2u, ... }; // N variants
match (*self, *__arg_0) {
  (A(ref __self_0), A(ref __arg_1_0)) => /* recur */,
  (B(ref __self_0), B(ref __arg_1_0)) => /* recur */,
  (C(ref __self_0), C(ref __arg_1_0)) => /* recur */,
  // as above, N variants, each bounded by complexity of the tuple of its variant.
  ...
  (_, _) => discrim_self.cmp(discrim_arg_0)
}
