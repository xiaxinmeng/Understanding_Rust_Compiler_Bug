rust
let mut it = my_map.into_iter();
it.by_ref().min();
println!("{:?}", it.next());
