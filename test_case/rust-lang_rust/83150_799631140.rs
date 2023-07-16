rust
#![type_length_limit="393"]
fn main() {
    func(&mut(0u8..1))
}

fn func<T: Iterator<Item = u8>>(iter: &mut T) {
    func(&mut iter.map(|x| x + 1))
}
