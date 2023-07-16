
#![feature(unboxed_closures)]
#![feature(core)]

struct Struct;

impl Struct {
    fn method_bad<F>(&self, f: F) -> bool where F: Fn(()) -> bool {
        self.method_bad(|t| f(t))
    }

    fn method_bad2<F>(&self, f: F) -> bool where F: Fn(()) -> bool {
        let f: &Fn(()) -> bool = &f;

        self.method_bad2(|t| f.call((t,)))
    }
}

fn main() {
    // Pick one
    Struct.method_bad(|_| true);
    //Struct.method_bad2(|_| true);
}

