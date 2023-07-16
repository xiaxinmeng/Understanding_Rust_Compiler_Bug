
error[E0281]: type mismatch: `[closure@src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:26:9: 26:14]` implements the trait `std::ops::Fn<()>`, but the trait `std::ops::Fn<usize>` is required
  --> src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:26:5
   |
26 |     bar(|| {});
   |     ^^^ ----- implements `std::ops::Fn<()>`
   |     |
   |     expected usize, found ()
   |     requires `std::ops::Fn<usize>`
   |
   = note: required by `bar`
