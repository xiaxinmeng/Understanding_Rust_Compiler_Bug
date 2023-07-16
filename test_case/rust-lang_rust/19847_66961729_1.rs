 rust
fn main() {
}

fn from_str_iter(iter: Iterator<&str>) -> Option<Command> { Command::from_str_iter(iter) }

struct Command<'s> {
    foo: int,
}

impl <'i>Command<'i> {
    fn from_str_iter(iter: Iterator<&str>) -> Option<Command> {
    }
}
