
#[deny(dead_assignment)];
fn main() {
    let mut x = 1;
    let f: &fn() -> int = || { x + 20 };
    assert_eq!(f(), 21);
    x += 1;
    assert_eq!(f(), 22);
}
