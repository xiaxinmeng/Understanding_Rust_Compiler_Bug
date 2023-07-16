rust
let mut a = 1..=3;
for i in a.by_ref() { println!("1: {:?}", i); }
a.start = 1;
a.end = 3;
// The following loop won't run at all.
for i in a.by_ref() { println!("2: {:?}", i); }
