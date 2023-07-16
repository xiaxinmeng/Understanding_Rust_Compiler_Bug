rust
let values = [1, 2, 3, 4];

let three: &[i32; 3] = values[..3].try_into().unwrap();

// Or even:
let last: &[i32; 2] = values[2..4].try_into().unwrap();
