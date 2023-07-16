rust
#[link_section = "foo"]
static A: u8 = 8;

fn main() {
    println!("{}", A);
}
