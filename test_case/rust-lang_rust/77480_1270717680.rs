rust
let v = vec![1, 2, 3];

v.get(1); // Some(2)
v.get(1)!; // 2
unsafe{
v.get(1); // blazing fast 2
}
v.get(4); // None
v.get(4)!; // panic, note exclamation mark
unsafe{
v.get(4);
} // blazing fast UB
