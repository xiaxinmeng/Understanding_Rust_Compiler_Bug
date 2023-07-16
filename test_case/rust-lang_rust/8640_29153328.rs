 rust
#[allow(unused_imports)];

mod foo {
    use baz::bar;
    mod bar {}
}
mod baz { pub mod bar {} }

fn main() {}
