
error[E0623]: lifetime mismatch
  --> $DIR/ex3-both-anon-regions.rs:12:12
   |
11 | fn foo<'a, 'b>(x: &mut Vec<&'a u8>, y: &'b u8) {
   |                            ------      ------ these two types are declared with different lifetimes...
12 |     x.push(y);
   |            ^ ...but data from `y` flows into `x` here
error: aborting due to previous error
