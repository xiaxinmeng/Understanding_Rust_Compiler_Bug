`rust
struct Foo {
    a: usize,
    b: &'static u32,
}

fn main() {
    Box::pin(async {
        Foo {
            b: &42,
            a: async { 0 }.await,
        };
    });
}

