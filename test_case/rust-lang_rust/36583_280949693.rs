
rustc 1.16.0-beta.2 (bc15d5281 2017-02-16)
error[E0423]: expected function, found type alias `Bar`
 --> <anon>:3:18
  |
3 |     let x: Bar = Bar(1);
  |                  ^^^ did you mean `Bar { /* fields */ }`?

error: aborting due to previous error
