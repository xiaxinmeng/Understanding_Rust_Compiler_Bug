rust
#[derive(PartialEq)]
enum E { A }
const E_SL: &[E] = &[E::A];

fn main() {
    match &[][..] {
        E_SL => {}
    }
}
