
error[E0277]: the trait bound `&&(&str, &std::ops::Fn(i32) -> bool): tool::sequence::Cons` is not satisfied
  --> src\lib.rs:52:27
   |
52 |         .filter(apply(|c| second(c), i))
   |                           ^^^^^^ trait `&&(&str, &std::ops::Fn(i32) -> bool): tool::sequence::Cons` not satisfied
   |
   = help: the following implementations were found:
   = help:   <(A, B) as tool::sequence::Cons>
   = help:   <&'a (A, B) as tool::sequence::Cons>
   = help:   <&'a mut (A, B) as tool::sequence::Cons>
   = note: required because of the requirements on the impl of `tool::sequence::Second` for `&&(&str, &std::ops::Fn(i32) -> bool)`
   = note: required by `tool::second`
