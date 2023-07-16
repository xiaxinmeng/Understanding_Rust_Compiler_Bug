rust
#[derive(PartialEq, Eq)]
struct Hi(i32);

const fn hi() -> Hi { Hi(10) }
const C: Hi = hi();

fn main() {
    match Hi(10) {
        self::C => println!("matched"),
        _ => println!("other"),
    }
}
