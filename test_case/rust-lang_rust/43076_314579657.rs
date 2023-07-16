rust
struct A(u32);

fn g1() -> impl Generator<Yield=(), Return=()> {
    move || {
        let a = A(33);
        let inner = a.g2();
        while let State::Yielded(_) = inner.resume(()) {
            yield
        }
    }
}

impl A {
    fn g2(&self) -> impl Generator<Yield=(), Return=()> {
        let x = self.0;
        move || {
            println!("use it {:?}", x);
            yield
        }
    }
}
