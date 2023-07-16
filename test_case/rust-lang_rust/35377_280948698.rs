
rustc 1.16.0-beta.2 (bc15d5281 2017-02-16)
error[E0532]: expected unit struct/variant or constant, found struct variant `A::V`
 --> <anon>:7:9
  |
7 |         A::V => {}
  |         ^^^^ did you mean `A::V { /* fields */ }`?

error: aborting due to previous error
