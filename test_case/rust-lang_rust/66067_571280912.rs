
error[E0423]: expected function, tuple struct or tuple variant, found struct `Triplet`
 --> src/main.rs:5:19
  |
5 |     triplets.push(Triplet(0, 0, 1.0));
  |                   ^^^^^^^
  |                   |
  |                   constructor is not visible here due to private fields
  |                   help: a local variable with a similar name exists: `triplets`
