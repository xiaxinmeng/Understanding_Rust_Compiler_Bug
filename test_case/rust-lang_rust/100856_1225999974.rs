rust
// Sort using a completely random comparison function.
// This will reorder the elements *somehow*, but won't panic.
let mut v = [0; 500];
for i in 0..v.len() {
    v[i] = i as i32;
}
v.sort_by(|_, _| *[Less, Equal, Greater].choose(&mut rng).unwrap());
v.sort();
for i in 0..v.len() {
    assert_eq!(v[i], i as i32);
}
