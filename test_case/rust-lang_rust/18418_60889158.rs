 rust
struct Pair<A> {
    a: A,
}

trait Printable {
    fn print(&self) {}
}

impl<A: Printable> Printable for Pair<A> {}

// inlining/removing these stops the error
fn pair<A>(a: A) -> Pair<A> {
    Pair{ a : a }
}
fn print<A: Printable>(t: Pair<A>) {}

fn main() {
    print(pair(1));
}
