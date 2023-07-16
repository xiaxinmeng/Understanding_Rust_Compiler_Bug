 rust
mod x {
    pub struct Foo { pub x: u32 }

    struct Bar { _x: u32 }

    impl Foo {
        pub fn foo(&self, _x: Self, _y: Bar) { }
        pub fn bar(&self) -> Bar { Bar { _x: self.x } }
    }
}

pub fn main() {
    let f = x::Foo { x: 4 };
    let b = f.bar();
    f.foo(x::Foo { x: 5 }, b);

}

