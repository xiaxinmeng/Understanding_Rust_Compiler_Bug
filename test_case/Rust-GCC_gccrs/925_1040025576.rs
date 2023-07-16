
struct S;
trait A {
    fn foo(&self);
}

impl A for S {
    fn foo(&self) {}
}

impl S {
    fn foo(&self) {}
}

fn test() {
    let a = S;
    a.foo();
}
