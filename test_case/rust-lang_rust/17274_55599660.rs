
fn apply<A,B,C>(f: |A,B|->C, t: (A,B)) -> C {
    let (a,b) = t;
    f(a,b)
}

fn main() {
    apply(sum, prepare_args());
}
