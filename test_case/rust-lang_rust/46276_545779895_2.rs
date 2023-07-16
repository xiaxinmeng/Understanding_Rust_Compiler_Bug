
error[E0368]: binary assignment operation `+=` cannot be applied to type `&mut i16`
 --> src/lib.rs:3:9
  |
3 |         item += 1;
  |         ----^^^^^
  |         |
  |         cannot use `+=` on type `&mut i16`
help: `+=` can be used on 'i16', you can dereference `item`
  |
3 |         *item += 1;
  |         ^^^^^
