rust
error[E0308]: mismatched types
  --> src/main.rs:18:9
   |
18 |         ($(dbg!($val)),+,)
   |         ^^^^^^^^^^^^^^^^^^ expected u8, found tuple
...
23 |     let _: u8 = dbg!(1,);
   |                 -------- in this macro invocation
   |
   = note: expected type `u8`
              found type `({integer},)`
