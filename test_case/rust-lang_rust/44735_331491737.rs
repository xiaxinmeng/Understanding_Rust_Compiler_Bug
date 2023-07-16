
error[E0631]: type mismatch in closure arguments
  --> E0631.rs:17:5
   |
17 |     foo(|_: isize| {});
   |     ^^^ ------------- found signature of fn(isize) -> _
   |     |
   |     expected signature of fn(usize) -> _
   |
   = note: required by `foo`

error[E0631]: type mismatch in closure arguments
  --> E0631.rs:18:5
   |
18 |     bar(|_: isize| {});
   |     ^^^ ------------- found signature of fn(isize) -> _
   |     |
   |     expected signature of fn(usize) -> _
   |
   = note: required by `bar`
