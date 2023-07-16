rust
warning: unused variable `i` in the lifetime starting at 1:1 and ending at 5:1
 --> src/main.rs:2:9
1 |  / fn main() {
...  |
2 |  |    let i = 3; // Lifetime for `i` starts. ────────────────┐
3 |  |     //                                                    │
...  |
4 |  |     //                                                    │
5 |  | }   // Lifetime ends. ────────────────────────────────────┘  
  |  |_^
  |  help: if the variable is meant to be unused, consider prefixing with an underscore: `_i`
  |
  = note: `#[warn(unused_variables)]` on by default

