
error[E0308]: mismatched types
  --> src/ice.rs:19:20
   |
19 |               return m1::S1 {
   |  ____________________^
20 | |                 v1: format!(""),
21 | |             }
   | |_____________^ expected enum `std::result::Result`, found struct `ice::m1::S1`
   |
   = note: expected enum `std::result::Result<ice::m1::S1, ()>`
            found struct `ice::m1::S1`
help: try using a variant of the expected enum
   |
19 |             return Ok(m1::S1 {
20 |                 v1: format!(""),
21 |             })
   |
