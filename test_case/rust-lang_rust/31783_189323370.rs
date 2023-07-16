
pub use(path1) foo::*; // (1)

mod foo {
    pub(path2) use bar::*;
    pub(path3) struct Baz;
}
