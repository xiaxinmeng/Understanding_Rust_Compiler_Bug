rust
let mut i = 1;
let mut something = 0;
loop {
    println!("i is {}", i);
    if i > 100 {
        something = i;
        break;
    }
    i *= 2;
}
assert_eq!(something, 128);
