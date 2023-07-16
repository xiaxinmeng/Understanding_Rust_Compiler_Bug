rust
existential type A: Sized;

fn foo() -> A {
    (|x| ()) as for<'a> fn(&'a i32)
}

fn main() {}
