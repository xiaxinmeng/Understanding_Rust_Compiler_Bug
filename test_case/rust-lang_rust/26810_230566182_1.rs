
error: mismatched types [--explain E0308]
 --> <anon>:3:27
  |>
3 |> const C: (usize, A) = (1, [257]);
  |>                           ^^^^^ expected an array with a fixed size of 257 elements, found one with 1 elements
note: expected type `[u8; 257]`
note:    found type `[u8; 1]`
