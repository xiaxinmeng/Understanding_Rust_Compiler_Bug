
error[E0061]: this method takes 1 argument but 4 arguments were supplied
 --> fuzz_input.rs:2:15
  |
2 |     let a = 0.le('r',3.+0,3.+0,                      3);
  |               ^^ ---      ----                       - unexpected argument of type `{integer}`
  |                  |        |
  |                  |        unexpected argument
  |                  unexpected argument of type `char`
  |
2

help: remove the extra arguments
  |
2 -     let a = 0.le('r',3.+0,3.+0,                      3);
2 +     let a = 0.le(,3.+0);
  |
