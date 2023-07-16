rust
use std::io;
use std::mem;

fn main() {
    let throw_away = io::stdout();
    mem::forget(throw_away);
    print!("Hello, world!");
}
