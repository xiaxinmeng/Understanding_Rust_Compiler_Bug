
fn main() -> () {
    let a = ~(1, 2);
    test(a);
}

fn test<T,U>(a : &(T, U)) {
    let (ref _x, ref _y) = *a;
}
