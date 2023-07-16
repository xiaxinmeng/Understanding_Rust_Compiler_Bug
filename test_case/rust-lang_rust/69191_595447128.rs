rust
pub enum Void {}

enum UninhabitedUnivariant {
    Variant(Void),
}

fn main() {
    let seed: UninhabitedUnivariant = None.unwrap();
    match seed {
        UninhabitedUnivariant::Variant(_x) => {}
    }
}
