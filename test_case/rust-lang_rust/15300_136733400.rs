 rust
let mut data = vec![1, 2, 3];

anyfunction(data.remove(0), &mut data);  // Ok
anyfunction(&mut data, data.remove(0));  // Not Ok
