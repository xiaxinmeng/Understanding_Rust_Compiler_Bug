rust
if v.iter().is_sorted() {
  for i in v.iter().map(|i| i* 2) {
    println!("{}", i);
  }
} 
