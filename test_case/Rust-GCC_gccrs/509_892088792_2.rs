
mod A {
    pub struct Foo {
        pub f: i32,
    }
}

fn main() {
    let _b = A::Foo {
        f: 33i32,
    };
}
