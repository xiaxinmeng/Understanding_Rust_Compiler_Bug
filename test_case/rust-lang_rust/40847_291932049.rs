rust
fn m_use() {
    mod foo {
        pub fn f() {}
    }
}
use fn<m_use>::foo; // assuming we could get `foo` to "escape" the block,
foo::f(); // `f` would resolve fine.
