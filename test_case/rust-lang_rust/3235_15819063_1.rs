
fn main() -> () {
    let a = ~(1, 2);
    test(a);
}

fn test<T,U>(a : &(T, U)) {
    let (_a, _b) = match *a {
        (ref a, ref b) => (a, b)
    };
}
