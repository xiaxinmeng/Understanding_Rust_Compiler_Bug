 rust
struct Generic<T>(T);
type Specific = Generic<usize>;

fn main() {
    let x = Generic::<usize>(42);
    let y = Specific(42);
}
