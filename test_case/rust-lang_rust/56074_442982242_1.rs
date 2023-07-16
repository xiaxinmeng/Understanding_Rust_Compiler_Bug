rust
let iter = Box::new(mv.iter_over(mis)) as Box<dyn Iterator<Item = &'a String>>;
for s in iter {
