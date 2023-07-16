rust
pub fn foo<'a, T>(s: &'a mut ()) where &'a mut (): Clone {
    <&mut () as Clone>::clone(&s);
}

fn main() {}
