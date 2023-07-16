
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 --> src/main.rs:4:16
  |
1 | struct S(u8, u8);
  | -------- fn(u8, u8) -> S {S} defined here
...
4 |     let _: S = S;
  |            -   ^ expected struct `S`, found fn item
  |            |
  |            expected due to this
  |
  = note: expected struct `S`
            found fn item `fn(u8, u8) -> S {S}`
help: use parentheses to construct this tuple struct
  |
4 |     let _: S = S(/* u8 */, /* u8 */);
  |                 ++++++++++++++++++++
