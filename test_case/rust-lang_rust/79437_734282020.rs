
fn foo() -> Take {
    Take(42)
}

struct Take(Take);

fn main() {}
