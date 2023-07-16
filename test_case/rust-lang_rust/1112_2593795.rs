
type x<T> = { a: T, b: bool, c: bool };

fn main() {
    let x: x<int> = { a: 3, b: false, c: true };
    assert x.c; // passes
    bar(x);
}

fn bar<T>(x: x<T>) {
    assert x.c; // fails
}
