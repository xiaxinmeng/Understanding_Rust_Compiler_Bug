 rust
enum Category {
    Bar = 4,
}
static i: int = Bar as int;

fn main () {
    let x: [int, ..i] = [1, 2, 3, 4];
}
