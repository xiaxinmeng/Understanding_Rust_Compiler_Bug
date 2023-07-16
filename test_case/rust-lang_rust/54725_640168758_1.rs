rust
use no_hygiene::no_hygiene;

macro_rules! weird {
    () => {
        println!("{}", no_hygiene!(my_ident));
    }
}

fn main() {
    let my_ident = "Hello";
    weird!();
}
