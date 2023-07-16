rust
struct A;

fn my_fn<'a>(_: &'a [A]) {}

const TEST: for<'a> fn(&'a [A]) = my_fn;

struct B { f: for<'a> fn(&'a [A]) }

fn main() {
    let s = B { f: my_fn };
    match s {
        B { f: TEST } => {}
        _ => {}
    };
}
