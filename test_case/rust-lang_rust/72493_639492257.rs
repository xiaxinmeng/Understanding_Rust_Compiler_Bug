
error[E0119]: conflicting implementations of trait `Trait` for type `for<'r> fn(fn(&'r ()))`:
  --> coherence-fn-covariant-bound-vs-static.rs:16:1
   |
15 | impl Trait for for<'r> fn(fn(&'r ())) {}
   | ------------------------------------- first implementation here
16 | impl<'a> Trait for fn(fn(&'a ())) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `for<'r> fn(fn(&'r ()))`
   |
   = note: this behavior recently changed as a result of a bug fix; see rust-lang/rust#56105 for details
