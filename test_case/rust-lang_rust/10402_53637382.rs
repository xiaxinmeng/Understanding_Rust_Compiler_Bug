 rust
enum Greeting {
    Hello,
    ByeBye
}

fn main() {
    let that = Hello;
    match that {
        Hello => (),
        NonExistent => (),
        ByeBye => ()
    }
}
