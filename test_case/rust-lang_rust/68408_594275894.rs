rust
// struct-like variant, referred to using Self

#[derive(Debug)]
enum E {
    A { x: () },
}

impl E {
    fn mk_e() -> Self {
        Self::A { x: () }
    }
}

// struct-like variant, referred to using type name

#[derive(Debug)]
enum F {
    B { x: () },
}

impl F {
    fn mk_f() -> Self {
        F::B { x: () }
    }
}

// tuple variant, referred to using Self

#[derive(Debug)]
enum G {
    C(()),
}

impl G {
    fn mk_g() -> Self {
        Self::C(())
    }
}

fn main() {
    dbg!(E::mk_e());
    dbg!(F::mk_f());
    dbg!(G::mk_g());
}
