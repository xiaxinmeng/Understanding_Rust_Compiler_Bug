
struct S {field: int}

mod a {
    priv trait A { fn x(); }
    impl S : A { fn x(){} }
}

mod b {
    priv trait B { fn x(); }
    impl S : B { fn x(){} }
}

fn main() {
    let s = S {field: 3};
    s.x();
}
