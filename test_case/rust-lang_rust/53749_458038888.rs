rust
let mut file = File::open(path).unwrap();
let mut s = String::new();
file.read_to_string(&mut s).unwrap();
String::leak(s)
