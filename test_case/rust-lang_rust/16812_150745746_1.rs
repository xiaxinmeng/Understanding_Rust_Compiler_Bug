 rust
trait T {}

fn main() {
    let x: &(T, Iterator<Item = u32>) = panic!();
}
