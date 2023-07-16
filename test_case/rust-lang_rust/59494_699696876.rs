rust
fn t7p<A,B>(g: impl Fn(A) -> B) -> impl Fn(A) -> B {
    g
}

fn unify<A,B,C>(f: impl Fn(A) -> B, g: impl Fn(A) -> C) {}

fn unify2<A,B,C>(f:impl Fn(B) -> C, g: impl Fn(A) -> B) {}

fn main() {
    let f = |(_, _)| {  };
    let g = |(a, _)| { a };

    unify2(f, g);

    let t = |env| { |a| { |b| { t7p(g)(((env, a), b)) }}};

    unify(t, t7p(g));
}
