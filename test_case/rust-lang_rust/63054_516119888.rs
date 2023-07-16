rust
let mut i = 0;
for _ in 0..4 {
    println!("{}", i);
    i += 1; // this should not warn
}
