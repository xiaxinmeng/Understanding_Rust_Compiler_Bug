rust
fn add(x: i32, y: i32) -> i32 { x + y }

fn main() {
    let items = vec![1, 2, 3];
    items.into_iter()
        .enumerate()
        .map(|tuple| add(tuple...))
        ...
}
