 rust
// other.rs
pub use self::bar::baz;

pub mod foo {

    struct Foo { x: int }
    pub fn mk_foo() -> Foo { Foo { x: 1 } }

}

mod bar { pub fn baz() { println("escaped!!") } }
