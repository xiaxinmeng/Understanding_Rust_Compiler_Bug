rust
struct Foo<A, B>(A, B);

fn foo<A: Display, B: Foo<u32, u64>, C: Foo<A, B>>(arg: A) {}
