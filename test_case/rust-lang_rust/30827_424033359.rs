rust
mod foo {
    #[deprecated(note = "use Foo instead")]
    pub use self::Foo as Bar;

    pub struct Foo(pub i32);
}

use self::foo::Bar; // <- deprecrated, use Foo instead
