rust
trait T {}

struct L<A = Box<dyn T>> {
    value: A,
}

async fn test1(l: L, s1: &str, s2: &str) {
}
