rust
struct A;

fn my_fn(_: *const [A]) {}

const TEST: fn(*const [A]) = my_fn;

struct B { f: fn(*const [A]) }

fn main() {
    let s = B { f: my_fn };
    match s {
        B { f: TEST } => {}
        _ => {}
    };
}
