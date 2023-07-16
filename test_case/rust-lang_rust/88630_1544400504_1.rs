rust
impl PartialEq<Foo> for u8 {
    fn eq(&self, other: &Foo) -> bool {
        todo!()
    }
}

struct Foo;

fn innocent_bystander() {
    "x".as_bytes() != &[];
}

fn main() {}
