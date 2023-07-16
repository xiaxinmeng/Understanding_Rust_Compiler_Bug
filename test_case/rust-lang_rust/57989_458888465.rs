
struct A {
    x: i32,
}

impl A {
    fn f(&self) {
        let g = &|| self.f();
        self.x = 0;
        g();
    }
}

fn main() {}
