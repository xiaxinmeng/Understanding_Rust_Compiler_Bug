
rustc 1.13.0 (2c6933acc 2016-11-07)
error: `bar` does not live long enough
 --> <anon>:4:12
  |
4 |     foo = &bar;
  |            ^^^ does not live long enough
5 | }
  | - borrowed value dropped before borrower
  |
  = note: values in a scope are dropped in the opposite order they are created

error: aborting due to previous error
