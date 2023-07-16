rust
let mut s = 0;
let range = 0 .. 10;

println!("{}", s);
for i in range {
    s += i;
    println!("{}", s);
}
for i in range.rev() { // currently errors
    s -= i;
    println!("{}", s);
}
