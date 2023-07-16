rust
match x {
    ..=-1 => println!("what? this wasn't supposed to be negative"),
    ..=15 =>  println!("fits in a nybble"),
    ..=127 => println!("fits in an i8"),
    ..=255 => println!("fits in a u8"),
}
