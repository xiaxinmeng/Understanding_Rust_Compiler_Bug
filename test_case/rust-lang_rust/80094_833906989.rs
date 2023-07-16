rust
let x = [1, 2, 3];
let y = [4, 5, 6];

let z: [_; 3] = x
   .into_iter_fixed()
   .zip(y)
   .map(|(a, b)| a + b)
   .collect();
