
error[E0593]: closure takes 0 arguments but 1 argument is required
  --> src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:26:5
   |
26 |     bar(|| {});
   |     ^^^ ----- takes 0 arguments
   |     |
   |     expected closure that takes 1 argument
   |
   = note: required by `bar`
