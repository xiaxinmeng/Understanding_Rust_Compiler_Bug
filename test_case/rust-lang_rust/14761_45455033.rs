 rust
#[start]
fn main(_: int, _: **u8) -> int {
    (bytes!("TEXT!").as_ptr()) as int
}
