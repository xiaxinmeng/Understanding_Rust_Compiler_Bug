
fn main() {
    let foo = fn&(c: char) -> bool { c == 'a' };
    let a = &"something".all(foo);
}
