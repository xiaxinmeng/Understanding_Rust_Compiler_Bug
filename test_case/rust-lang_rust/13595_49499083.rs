 rust
struct A<'a> {
    func: &'a fn() -> Option<int>
}

impl<'a> A<'a> {
    fn call(&self) -> Option<int> {
        (*self.func)()
    }
}

fn foo() -> Option<int> {
    None
}

fn create() -> A<'static> {
    A { func: &foo }
}

fn main() {
    let a = create();
    a.call();
}
