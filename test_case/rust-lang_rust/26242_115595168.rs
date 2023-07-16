 rust
pub use foo::Foo;

use Foo as FooBar;

mod foo {
    pub use self::bar::*;
    mod bar {
        pub struct Foo;
    }
}
