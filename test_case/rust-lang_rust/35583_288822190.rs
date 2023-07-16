rust
error[E0532]: expected tuple struct/variant, found struct variant `A::B`
 --> <anon>:9:5
  |
9 |     A::B(1) => "uh oh",
  |     ^^^^ did you mean `A::B { /* fields */ }`?
