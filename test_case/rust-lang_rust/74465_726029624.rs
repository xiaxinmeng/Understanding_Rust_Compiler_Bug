rust
let x = 92;
let works1: = Lazy::new(|| x.to_string());
let broken: Lazy<String> = Lazy::new(|| x.to_string());
let works2: Lazy<String, _> =  Lazy::new(|| x.to_string());
