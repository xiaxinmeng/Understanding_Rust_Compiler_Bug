 rust
mod test_mod {
   enum A {
        Foo,
    }
}

type Alias = test_mod::A;

fn main () {
    let _v = Alias::Foo; // Compile error
}
