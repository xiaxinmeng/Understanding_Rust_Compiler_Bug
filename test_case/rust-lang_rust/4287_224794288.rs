 rust
enum Perfect<T> { Tip(T), Fork(Box<Perfect<(T, T)>>) }

fn main() {
    let _ = Perfect::Tip(42);
}
