
struct Take(Self);
fn foo() -> impl Clone {
    Take(42)
}

fn main() {}
