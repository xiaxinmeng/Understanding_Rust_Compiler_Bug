rust
struct S(u32, Vec<i32>);

fn foo(x: &S) {
    match x {
        S(&y, v) => {
            // match arm goes here
        }
    }
}
