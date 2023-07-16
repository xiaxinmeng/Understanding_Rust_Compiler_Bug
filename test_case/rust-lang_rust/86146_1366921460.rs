
error[E0599]: the method `clone` exists for struct `Vec<Vec<Unclonable>>`, but its trait bounds were not satisfied
 --> src/main.rs:5:26
  |
5 |     let clone = original.clone();
  |                          ^^^^^ method cannot be called on `Vec<Vec<Unclonable>>` due to unsatisfied trait bounds
 --> /rustc/92c1937a90e5b6f20fa6e87016d6869da363972e/library/alloc/src/vec/mod.rs:400:1
  |
  = note: doesn't satisfy `Vec<Unclonable>: Clone`
  |
  = note: doesn't satisfy `Vec<Vec<Unclonable>>: Clone`
  |
  = note: the following trait bounds were not satisfied:
          `Vec<Unclonable>: Clone`
          which is required by `Vec<Vec<Unclonable>>: Clone`
