rust
#[derive(Default)]
struct S;

impl S {
    fn f() -> S {
        r#Self::default() // Is this an error?
    }  
}
