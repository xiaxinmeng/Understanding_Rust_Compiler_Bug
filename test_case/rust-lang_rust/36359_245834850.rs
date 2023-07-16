 rust
fn main() {
    let minus_1: i32 = 0xFF_FF_FF_FF_i32;

    if std::i32::MIN <= minus_1 && minus_1 <= std::i32::MAX {
        println!("{}", minus_1);
    }
}
// =>
// -1
