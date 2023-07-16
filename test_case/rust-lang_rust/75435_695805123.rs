rust
// before
let mut file = File::open("address.txt")?;
let mut s = String::new();
file.read_to_string(&mut s)?;
let foo: SocketAddr = s.parse()?;

// after
let foo: SocketAddr = fs::read_to_string("address.txt")?.parse()?;
