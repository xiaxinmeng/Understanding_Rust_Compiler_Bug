 rust
trait Ice<'a, T=&'a i32> {
    fn foo() -> T;
}

fn foo<'a, T>() -> T where T: Ice<'a> {
    Ice::foo()
}
