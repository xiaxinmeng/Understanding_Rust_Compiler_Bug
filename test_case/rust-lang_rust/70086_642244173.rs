rust
let a = Some(7);
let b = Some(true);
let out = a.iter().zip(b).map(|(a, b)| s * o as u8).next(); //stable
let out = a.zip(b).map(|(a, b)| a * b as u8); // #![feature(option_zip)]
let out = a.zip_with(b, |a, b| a * b as u8); // #![feature(option_zip)]
let out: Option<_> = try { a? * b? as u8 }; // #![feature(try_blocks)]
