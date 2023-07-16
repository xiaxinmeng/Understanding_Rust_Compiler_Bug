rust
// before
print!("Guess a number: ");
let mut input = String::new();
io::stdin().read_line(&mut input)?;
let input = input.trim_end_matches(&['\n', '\r'][..]);
let num: u16 = input.parse();

// after
print!("Guess a number: ");
let num: u16 = io::read_line()?.parse()?;
