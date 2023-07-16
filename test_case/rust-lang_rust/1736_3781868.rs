
fn fst<T: copy, U: copy>(x: (T, U)) -> T {
    let (x, _) = x; x
}

fn main() {
    let u = (10, 10), v = ({a: 10, b: 20}, 1u), i = 0;
    while i < 3000000 {
        fst(u); fst(v);
        i += 1;
    }
}
