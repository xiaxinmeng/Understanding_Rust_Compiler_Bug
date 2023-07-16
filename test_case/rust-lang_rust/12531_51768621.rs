 rust
// (inline mods are isomorphic to separate files)
mod submod {
    mod submod {
        pub fn foo() {}
    }
}

fn main() {
  submod::foo();
}
