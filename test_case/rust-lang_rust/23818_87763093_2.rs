 rust
let output = io::stdout();
let output = output.lock();
print!("stdout locked"); // deadlock.
