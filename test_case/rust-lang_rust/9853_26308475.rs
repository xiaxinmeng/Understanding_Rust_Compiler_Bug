 rust
struct A {
    foo : ~fn(&mut A)
}

impl A {
    fn call(&mut self) {
        (self.foo)(self)
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
    let x = S { msg: ~"ok" };
    let f: ~fn(&mut A) = |a| { a.foo = |_| {}; println!("{:?}", x); }; // this frees closure environment via `a`, then accesses it
    let mut a = A { foo: f };
    a.call();
}
