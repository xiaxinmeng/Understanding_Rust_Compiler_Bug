 rust
macro_rules! foo { () => {} }
foo! {} // resolves to ::foo
#[macro_use] mod bar {
    macro_rules! foo { () => {} }
}
foo! {} // resolves to ::bar::foo
