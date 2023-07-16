
use std::fmt::Arguments;

struct Lol;

impl Lol {
    fn write_fmt(self, _: Arguments) -> std::fmt::Result {
        println!("called format");
        Ok(())
    }
}

fn main() {
    let l = Lol;
    write!(l, "yup").unwrap();
}
