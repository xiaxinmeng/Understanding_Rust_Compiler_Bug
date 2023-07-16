
error[E0308]: mismatched types
 --> src/main.rs:7:22
  |
7 |     BigRational::pow(BigRational::one(), 23);
  |                      ^^^^^^^^^^^^^^^^^^
  |                      |
  |                      expected `&Ratio<BigInt>`, found struct `Ratio`
  |                      help: consider borrowing here: `&BigRational::one()`
  |
  = note: expected reference `&Ratio<BigInt>`
                found struct `Ratio<BigInt>`
