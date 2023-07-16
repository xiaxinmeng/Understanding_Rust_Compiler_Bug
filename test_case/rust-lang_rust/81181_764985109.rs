rust
struct Foo;

impl PartialEq<Foo> for u32 {
    fn eq(&self, _: &Foo) -> bool {
        false
    }
}

fn main() {
    let x = 42_u32;
    let y = 0u8;
    
    assert!(x != y.into());
}
