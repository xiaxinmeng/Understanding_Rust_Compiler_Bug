rust
let mut v: Vec<_> = vec![1,3,2,4];
let not_sorted: bool = !v.into_iter().is_sorted();
if  not_sorted {
  v.sort();  // ERROR: v has been moved
}
