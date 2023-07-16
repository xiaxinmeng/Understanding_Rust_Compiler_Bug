rs
struct Bar<T>(T);

struct Inner(u8);

fn main() {
    let thing = Bar(Inner);
    thing.0.0();
}
