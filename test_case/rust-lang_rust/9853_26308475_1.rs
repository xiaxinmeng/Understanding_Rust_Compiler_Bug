 rust
struct A {
    foo : ~NotAClosure
}

impl A {
    fn call(&mut self) {
        self.foo.call(self);
    }
}

struct NotAClosure {
    s: Option<S>
}

impl NotAClosure {
    fn call(&self, a:&mut A) {
        a.foo.s = None;
        println!("{:?}", self);
    }
}

struct S {
    msg: ~str
}

impl Drop for S {
    fn drop(&mut self) {
        println!("drop {:?}", *self);
        self.msg = ~"dropped";
    }
}

fn main() {
    let mut a = A{ foo: ~NotAClosure { s: Some(S { msg: ~"ok" }) } };
    a.call();
}
