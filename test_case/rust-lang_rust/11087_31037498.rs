 rust
enum TryValue<T> {
    Empty,
    Disconnected,
    Data(T),
}

fn try_recv(&mut self) -> TryValue<T>;
