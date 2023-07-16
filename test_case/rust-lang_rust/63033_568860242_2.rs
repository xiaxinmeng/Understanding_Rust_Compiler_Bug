rust
use std::future::Future;

trait T {}

struct L<A> {
    value: A,
}

fn test1(l: L<Box<dyn T>>, s1: &str, s2: &str) -> impl Future<Output=()> {
    async {
    }
}
