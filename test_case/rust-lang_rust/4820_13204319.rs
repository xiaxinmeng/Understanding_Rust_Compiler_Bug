
fn min<T:Ord>(x: T, y: T) -> T {
    if x < y { x } else { y }
}

fn main() {
    assert (min(1, 2) == 1);
}
