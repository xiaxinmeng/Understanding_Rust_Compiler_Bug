rust
rustc 1.17.0-nightly (be760566c 2017-02-28)
error[E0423]: expected function, found struct `MyStruct`
 --> <anon>:4:5
  |
4 |     MyStruct();
  |     ^^^^^^^^ did you mean `MyStruct { /* fields */ }`?

error: aborting due to previous error
