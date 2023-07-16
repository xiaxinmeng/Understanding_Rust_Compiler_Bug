rust
struct Bar();

impl Bar {
    fn nv(&self, _: &()) -> usize { 42 }
}

fn foo(p: &Bar) {
    let _ = p[p.nv(&())];
}

fn main() {}
