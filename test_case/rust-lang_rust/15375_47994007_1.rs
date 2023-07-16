 rust
match *self {
  // each of these is ...
  A(ref __self_0) => match *__arg_0 { A(ref__arg_1_0) => /* recur */, B(_) => Less, C(_) => Less, D(_) => Less, ... },
  // ... N elements long
  B(ref __self_0) => match *__arg_0 { A(_) => Greater, B(ref__arg_1_0) => /* recur */, C(_) => Less, D(_) => Less, ... },
  // .. and there are N variants, yielding O(N^2) code.
  ...
}
