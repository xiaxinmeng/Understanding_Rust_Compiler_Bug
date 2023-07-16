rust
(0..5).take(3).eq(0..3);
// or
(0..5).zip(0..3).all(|(a, b)| a == b);
