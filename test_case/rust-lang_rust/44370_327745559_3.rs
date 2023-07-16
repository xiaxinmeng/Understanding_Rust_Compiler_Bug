rust
let v: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
let it = v.iter().map(|i| i* 2);
for i in it { println!("{}", i); }  // it is moved here
for i in it { println!("{}", i); }  // ERROR: it was already moved
