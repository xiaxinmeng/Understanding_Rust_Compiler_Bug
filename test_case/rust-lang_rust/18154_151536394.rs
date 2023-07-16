
enum Tag {
    Dynamic,
    Inline(u8),
    Static,
}

#[test]
fn f() {
    assert_eq!(Tag::Inline as usize, 1);
}

fn main() {}
