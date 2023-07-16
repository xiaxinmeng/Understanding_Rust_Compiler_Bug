 rust
let input = io::stdin();
let mut locked = input.lock();

print!("enter input: ");
let mut input = String::new();
try!(locked.read_line(&mut input));

println!("you entered: {}", input);
