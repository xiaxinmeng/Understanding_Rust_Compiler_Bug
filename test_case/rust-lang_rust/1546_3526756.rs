
fn foo<T: copy, U: copy>(pair: {a: T, b: U}) -> T {
    let x = pair;
    let w = x; // This is where it segfaults
    // log(error, x); // If you add this, x above is not a last use, and the problem goes away
    ret w.a;
}

fn main() {
    foo({a: 10u8, b: 20u8});
}
