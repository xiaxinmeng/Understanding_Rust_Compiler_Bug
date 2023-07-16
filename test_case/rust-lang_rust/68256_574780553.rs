
error: expected identifier, found `)`
  --> src/ice.rs:24:9
   |
24 |         }
   |         ^ expected identifier

error: mismatched closing delimiter: `}`
  --> src/ice.rs:24:9
   |
22 |         } else {
   |                - closing delimiter possibly meant for this
23 |             return Err(m1::E1::
   |                       - unclosed delimiter
24 |         }
   |         ^ mismatched closing delimiter

error[E0308]: mismatched types
  --> src/main.rs:1:1
   |
1  | / mod ice;
2  | | fn main() {
3  | | }
...  |
   |
   = note: expected enum `std::result::Result<ice::m1::S1, ()>`
            found struct `ice::m1::S1`
