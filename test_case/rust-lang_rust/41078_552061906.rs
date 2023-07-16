rust
struct S {}

fn filter<P>(predicate: P)
where
    P: Fn(&S) -> bool,
{
    predicate(&S{});
}

fn main() {
    // this works
    filter(|_s| true);

    // this also works
    fn cb1(_s: &S) -> bool {
        true
    }
    filter(cb1);

    // but this doesn't work
    let cb2 = |_s| true;
    filter(cb2);
}
