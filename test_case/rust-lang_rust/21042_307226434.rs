
rustc 1.17.0 (56124baa9 2017-04-24)
error[E0308]: mismatched types
 --> <anon>:5:10
  |
5 |     test(i);
  |          ^ expected enum `std::option::Option`, found integral variable
  |
  = note: expected type `std::option::Option<i32>`
             found type `{integer}`

error: aborting due to previous error
