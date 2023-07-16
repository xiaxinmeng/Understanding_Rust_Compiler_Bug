 Rust
enum Open {}
enum Closed {}

struct Door<State> {
    name: ~str
}

fn open(Door { name }: Door<Closed>) -> Door<Open> {
    Door { name: name }
}

fn main() {
    let d: Door<Open> = Door { name: ~"front" };
    open(d); // Fails (as it should)

    let d = Door::<Open> { name: ~"front" };
    open(d); // Should fail but doesn't
}
