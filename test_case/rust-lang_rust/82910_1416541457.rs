
error[E0599]: the method `join` exists for array `[&String; 2]`, but its trait bounds were not satisfied
 --> src/main.rs:4:29
  |
4 |     println!("{}", [&a, &b].join(","));
  |                             ^^^^ method cannot be called on `[&String; 2]` due to unsatisfied trait bounds
  |
  = note: the following trait bounds were not satisfied:
          `[&String]: Join<_>`
