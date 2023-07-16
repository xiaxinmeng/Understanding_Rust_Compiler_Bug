 rust
struct Parser<'a>(Box<FnMut(Parser) + 'a>);

fn main() {
    let x = Parser(Box::new(|_|{}));
}
