 rust
pub enum FragmentRepr {
    Boxed(Box<FragmentRepr>),
    Enum(Vec<FragmentRepr>),
}
fn bar() -> u32 { panic!(); }
pub fn foo(mut entry: &mut FragmentRepr) {
    loop {
        match bar() {
            1 => {
                let mut discrs = if let FragmentRepr::Enum(ref mut discrs) = *entry {
                    discrs
                } else {
                    return;
                };
                entry = &mut discrs[0];
            }
            _ => {
                entry = if let FragmentRepr::Boxed(ref mut contents) = *entry {
                    contents
                } else {
                    return;
                };
            }
        }
    }
}
fn main() {}
