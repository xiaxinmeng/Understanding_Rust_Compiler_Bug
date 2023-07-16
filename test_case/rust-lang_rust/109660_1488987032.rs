rust
let start = SystemTime::now();
do_stuff();
println!("Time it took to run do_stuff: {:?}", SystemTime::now().duration_since(start).unwrap())
