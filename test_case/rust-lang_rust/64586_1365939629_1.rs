rust
struct S(u32);

fn foo(x: &S) {
    let _: &u32 = match *x {
        S(y, ref v) => {
            // match arm goes here
        }
    }
}
