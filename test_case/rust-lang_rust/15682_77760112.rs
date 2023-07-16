
struct Foo(u8);

macro_rules! bit(
    ($_self:expr, $i:expr) => { $_self.0 & (1 << $i) != 0 };
);

impl Foo {
    fn is_bar(self) -> bool { bit!(self, 1) }
}

fn main() { 
    println!("{}", Foo(0x00).is_bar()); 
    println!("{}", Foo(0x02).is_bar());
}
