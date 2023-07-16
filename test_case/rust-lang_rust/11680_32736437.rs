 rust
mod foo {
    enum Private { pub Public }
}

fn main() { foo::Public; }
