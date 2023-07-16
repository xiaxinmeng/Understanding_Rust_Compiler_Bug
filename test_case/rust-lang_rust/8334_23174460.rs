 rust
#[inline(never)]
fn doit_not_generic(a: f32) -> f32 {
    let mut a = a;
    do 1000000000.times {
        a = a * a;
    }

    a
}

#[inline(never)]
fn doit<N: Mul<N, N>>(a: N) -> N {
    let mut a = a;
    do 1000000000.times {
        a = a * a;
    }

    a
}


fn main() {
    assert!(doit_not_generic(2.0f32) == doit(2.0f32));
}
