
error[E0623]: lifetime mismatch
 --> src/main.rs:2:10
  |
1 | fn foo(x: Box<Fn(&u8, &u8)>, y: Vec<&u8>, z: &u8) {
  |                  ---  --- these references are not declared with the same lifetime...
2 |   y.push(z); 
  |          ^ ...but data from `z` flows into `y` here
