rust
struct S;

impl S {
    fn method(&self) -> bool {
        unimplemented!()
    }
}

fn get<T>() -> T {
    unimplemented!()
}

fn main() {
    match get() {
        x if x.method() => {}
        &S => {}
    }
}
