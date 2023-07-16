
  --> $DIR/ex3-both-anon-regions-return-type-is-anon.rs:17:5
   |
16 |   fn foo<'a>(&self, x: &i32) -> &i32 {
   |              -----     ---- this parameter and the return type are
                            declared with different lifetimes...
17 |     x
   |     ^ ...but data from `x` flows into `self` here

error: aborting due to previous error
