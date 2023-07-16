rust
struct S;

impl Drop for S {
    fn drop(&mut self) {
        println!("dropping");
    }
}

enum E {
    A,
    B((S, S)),
}

fn main() {
    let mut foo = Some(E::A);
    match foo {
        Some(E::A) => (),
        Some(E::B(_)) | None => return,
    }
    foo.insert(E::B((S, S)));
    match foo {
        Some(E::B((x, _))) => {}
        _ => {},
    };
}
