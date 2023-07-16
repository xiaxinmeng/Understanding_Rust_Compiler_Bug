 rust
enum MyOption<T> {
    MySome(T),
    MyNone,
}

fn main() {
    match MySome(()) {
        MySome { x } => (),
        _ => (),
    }
}
