rust
if v.into_iter().is_sorted() { // vec is consumed here
  for i in v.iter().map(|i| i * 2) {
    println!("{}", i);
  }
} 
