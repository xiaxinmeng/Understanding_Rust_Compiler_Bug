
error[[E0599]](https://doc.rust-lang.org/nightly/error-index.html#E0599): the method `neg` exists for struct `Foo<T>`, but its trait bounds were not satisfied
  --> src/lib.rs:20:7
   |
3  | pub struct Foo<T>(pub T);
   | -------------------------
   | |
   | method `neg` not found for this
   | doesn't satisfy `Foo<T>: Neg`
...
20 |     f.neg()
   |       ^^^ method cannot be called on `Foo<T>` due to unsatisfied trait bounds
   |
