rust
pub struct Bar {
    pub f: i32,
}

mod foomod {
    pub struct Foo {
        pub prout: i32,
    }

    pub fn totoprout() -> i32 {
        return 3i32;
    }
}

pub fn toto() {}

fn test() -> Bar {
    Bar { f: 23i32 }
}

fn main() {
    let a;
    a = foomod::Foo { prout: 3i32 };

    toto();

    let b;
    b = foomod::totoprout();
}
