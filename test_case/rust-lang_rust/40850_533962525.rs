
error[E0608]: cannot index into a value of type `(u32, u64, isize)`
 --> src/main.rs:4:24
  |
4 |       println!("{}\n", x[i]);
  |                        ^^^^
  |
  = help: to access tuple elements, use tuple indexing syntax (e.g., `tuple.0`)
