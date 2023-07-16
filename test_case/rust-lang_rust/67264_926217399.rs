rust
match x {
    ..=15 =>  println!("fits in a nybble"),
    ..=127 => println!("fits in an i8"),
    ..=255 => println!("fits in a u8"),
}
