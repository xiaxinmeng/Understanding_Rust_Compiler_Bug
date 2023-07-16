 Rust
let i = 0i32;

match i {
    x @ 10...100 => println!("{} is between 9 and 101", x),
    x => println!("{} isn't between 9 and 101", x)
};
