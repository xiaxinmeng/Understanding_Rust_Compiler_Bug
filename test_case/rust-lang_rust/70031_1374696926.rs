
error[[E0308]](https://doc.rust-lang.org/stable/error-index.html#E0308): mismatched types
 --> src/main.rs:4:9
  |
4 | /         for e in v.iter() {
5 | |             if *e == 0 {
6 | |                 return Err(e);
7 | |             }
8 | |         }
  | |_________^ expected enum `Result`, found `()`
  |
  = note:   expected enum `Result<_, &u32>`
          found unit type `()`
note: return type inferred to be `Result<_, &u32>` here
 --> src/main.rs:6:24
  |
6 |                 return Err(e);
  |                        ^^^^^^
help: try adding an expression at the end of the block
  |
8 ~         }
9 +         Ok(())
  |
