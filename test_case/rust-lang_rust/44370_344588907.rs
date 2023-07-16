rust
let mut v: Vec<_> = vec![1,3,2,4];
let not_sorted: bool = !v.iter().is_sorted();
if  not_sorted {
  v.sort();  // works
}
let not_sorted: bool = !v.iter_mut().is_sorted();
if not_sorted {
  v.sort(); // works
}
