rust
#[derive(MyProcMacro)]
struct S;

impl S {
    fn clone(&self) -> Self { /* appropriate manual implementation here */ }
}
