
error: mismatched types [--explain E0308]
 --> <anon>:5:13
  |>
5 |>         v = &mut Vec::new();
  |>             ^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found struct `std::vec::Vec`
note: expected type `&'static std::option::Option<std::vec::Vec<u32>>`
note:    found type `&mut std::vec::Vec<_>`
