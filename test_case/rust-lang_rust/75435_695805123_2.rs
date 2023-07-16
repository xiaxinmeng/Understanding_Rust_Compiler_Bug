rust
let retry_count = 3;

// before
print!("Guess the number ({} tries remaining): ", retry_count);
let num = io::read_line()?.parse();

// after
let prompt = format!("Guess the number ({} tries remaining): ", retry_count);
let num = io::read_prompt(format)?.parse()?;
