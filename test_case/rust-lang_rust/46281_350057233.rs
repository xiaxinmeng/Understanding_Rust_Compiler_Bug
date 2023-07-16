rust
println!("{}", <i16>::from_str_radix("-ff", 16).unwrap());
println!("{}", <i16>::from_str_radix("ffff", 16).unwrap_err());
