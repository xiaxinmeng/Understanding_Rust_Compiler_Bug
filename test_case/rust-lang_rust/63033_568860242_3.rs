rust
trait T {}

struct L<A> {
    value: A,
}

async fn test2(l: L<Box<dyn T>>, s1: &str) {
}

async fn test3(l: L<Box<dyn T>>) {
}
