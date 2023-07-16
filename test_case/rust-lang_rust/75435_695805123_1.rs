rust
// before
print!("Your name: ");
let name = io::read_line()?;

// after
let name = io::read_prompt("Your name: ")?;
