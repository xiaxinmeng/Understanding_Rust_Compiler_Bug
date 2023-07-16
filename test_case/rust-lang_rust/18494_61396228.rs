 rust
#![feature(unboxed_closures)]
#![feature(overloaded_calls)]

trait Callback<S, V> {
    fn invoke(self, s: S, v: V);
}

impl<S, V, F: FnOnce(V)> Callback<S, V> for F {
    fn invoke(self, _: S, v: V) {
        self.call_once((v,));
    }
}

impl<S, V> Callback<S, V> for fn(S, V) {
    fn invoke(self, s: S, v: V) {
        self(s, v);
    }
}

fn invoke<S, V, F: Callback<S, V>>(s: S, v: V, f: F) {
    f.invoke(s, v);
}

struct Foo {
    val: uint,
}

impl Foo {
    fn action(&self) {
        invoke(self, "zomg", move |:val| println!("val: {}", val));
        invoke(self, "hi2u", Foo::callback);
    }

    fn callback(&self, val: &'static str) {
        println!("self: {}; val={}", self.val, val);
    }
}

pub fn main() {
    let f = Foo { val: 123u };
    f.action();
}
