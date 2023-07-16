
error[[E0423]](https://doc.rust-lang.org/stable/error-index.html#E0423): cannot initialize a tuple struct which contains private fields
  --> src/main.rs:22:13
   |
22 |         id: SomeId(32),
   |             ^^^^^^ help: a tuple variant with a similar name exists: `Some`
   |
note: constructor is not visible here due to private fields
  --> src/main.rs:3:23
   |
3  |     pub struct SomeId(u32);    
   |                       ^^^ private field
