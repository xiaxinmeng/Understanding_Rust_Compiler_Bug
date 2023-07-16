 rust
fn main() {
    make_five(1);
}

fn make_five(_: int) -> int
{
    _ + 5  // error: unexpected token: `_`
}
