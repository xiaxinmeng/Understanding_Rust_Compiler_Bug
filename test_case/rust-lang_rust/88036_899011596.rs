rust
const fn one() -> i32 {
    1
}

struct S<const C: i32>;

fn main() {
    match S::<1> {
        S::<{one()}> => {}
    }
}
