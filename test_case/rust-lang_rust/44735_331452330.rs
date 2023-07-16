
error[E0631]: type mismatch in closure arguments
  --> src/test/ui/mismatched_types/closure-mismatch.rs:18:5
   |
18 |     baz(|_| ());
   |     ^^^ ------ found signature of fn(_) -> _
   |     |
   |     expected signature of for<'r> fn(&'r ()) -> _
   |
   = note: required because of the requirements on the impl of `Foo` for `[closure@src/test/ui/mismatched_types/closure-mismatch.rs:18:9: 18:15]`
   = note: required by `baz`
