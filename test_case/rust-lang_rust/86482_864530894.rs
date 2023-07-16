rust
match a & 0b11 {
    0b0 => println!("got 0!"),
    0b1 => println!("got 1!"),
    0b10 => println!("got 2!"),
    0b11 => println!("got 3!"),
    _ => unreachable!()
}
