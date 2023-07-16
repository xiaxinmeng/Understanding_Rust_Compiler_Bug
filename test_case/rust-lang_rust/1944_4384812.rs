
fn with<A:copy,B:copy>(a: A, b: B) -> (A, B) {
    (a, b)
}

fn main() {
    let pair = with(3u, "hi");
    assert pair == (3u, "hi");
}
