rust
struct A<F: Fn()> {
    f: F
}

impl<F: Fn()> A<F> {
    fn f(&self) {
        println!("`A::f` called");
    }
}

fn main() {
    let a = A { f: || println!("closure called") };
    a.f(); // `A::f` called
    (a.f)(); // closure called
}
