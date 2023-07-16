
fn main() {
    fn foo(c: char) -> bool { c == 'a' }
    let a = &"something".all(foo);
}
