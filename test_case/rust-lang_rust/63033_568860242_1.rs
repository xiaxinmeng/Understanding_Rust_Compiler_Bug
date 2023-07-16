rust
trait T {}

struct L<A> {
    value: A,
}

async fn test1(l: L<Box<dyn T>>, s1: &str, s2: &str) {
}
