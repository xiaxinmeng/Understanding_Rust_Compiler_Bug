rust
fn main() {
    // Works
    // enum Foo {A, B}

    enum SecretFoo {
        A,
        B,
    }
    type Foo = SecretFoo;
    
    // Works fine
    let x = Foo::A;

    // Doesn't work
    use Foo::*;
    let y = A;
}
